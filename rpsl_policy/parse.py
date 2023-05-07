from sys import stderr
from typing import Iterable

from pyparsing import ParseException, ParserElement

from .lex import action, afi, as_expr, mp_filter, mp_peering


def lex_with(lexer: ParserElement, string: str) -> dict | None:
    try:
        return lexer.parse_string(string).as_dict()
    except ParseException as err:
        print(f"{err} parsing `{string}`.", file=stderr)


def merge_afi(afis: Iterable[dict[str, str]]) -> list[tuple[str, str]]:
    afi_sets: dict[str, set[str]] = {}
    for afi_item in afis:
        version = afi_item["version"]
        cast = afi_item.get("cast", "any")
        entry = afi_sets.get(version, set())
        entry.add(cast)
        afi_sets[version] = entry
    afi_map: dict[str, str] = {}
    for key, afi_set in afi_sets.items():
        if "any" in afi_set or ("unicast" in afi_set and "multicast" in afi_set):
            afi_map[key] = "any"
        else:
            assert len(afi_set) == 1
            afi_map[key] = afi_set.pop()
    if (v4 := afi_map.get("ipv4")) and (v6 := afi_map.get("ipv6")) and (v4 == v6):
        return [("any", v4)]
    return [(key, value) for key, value in afi_map.items()]


def clean_action(
    actions_lexed: Iterable[dict[str, dict | list]]
) -> dict[str, str | list[str] | list[dict[str, str | list[str]]]]:
    """Clean up a stream of <action>s
    -> {[<assignee1>...]: str | list[str], [community]: list[{
        [method]: str, args: list[str]
    }], [<rp-attribute1>...]: list[{method: str, args: list[str]}]}"""
    cleaned = {}
    for action_lexed in actions_lexed:
        if assignment := action_lexed.get("assignment"):
            assert isinstance(assignment, dict)
            cleaned[assignment["assignee"]] = (
                assigned
                if (assigned := assignment.get("assigned"))
                else assignment["assigned-set"]
            )
        elif community := action_lexed.get("community"):
            assert isinstance(community, dict)
            community_entry = cleaned.get("community", [])
            community_entry.append(community)
            cleaned["community"] = community_entry
        elif add_community := action_lexed.get("add-community"):
            assert isinstance(add_community, list)
            community_entry = cleaned.get("community", [])
            community_entry.append({"method": "=", "args": add_community})
            cleaned["community"] = community_entry
        elif method_call := action_lexed.get("method-call"):
            assert isinstance(method_call, dict)
            rp_attribute = method_call.pop("rp-attribute")
            rp_entry = cleaned.get(rp_attribute, [])
            rp_entry.append(method_call)
            cleaned[rp_attribute] = rp_entry
    return cleaned


def clean_mp_filter_base(lexed: dict):
    """community -> {community: {[method]: str, args: list[str]}}
    policy-filter -> list[str | list[str]]
    mp_filter -> ..."""
    if "community" in lexed:
        return lexed
    if policy_filter := lexed.get("policy-filter"):
        return policy_filter
    return clean_mp_filter(lexed)


def clean_mp_filter(lexed: dict):
    """-> {(and | or: {left, right}) | not}
    | {community: {[method]: str, args: list[str]}}
    | list[str | list[str]]"""
    if inner := lexed.get("and"):
        return {
            "and": {
                "left": clean_mp_filter(inner["left"]),
                "right": clean_mp_filter(inner["right"]),
            }
        }
    if inner := lexed.get("or"):
        return {
            "or": {
                "left": clean_mp_filter(inner["left"]),
                "right": clean_mp_filter(inner["right"]),
            }
        }
    if inner := lexed.get("not"):
        return {"not": clean_mp_filter(inner)}
    return clean_mp_filter_base(lexed)


def clean_as_expr(lexed: dict):
    """ "-> str | {and | or | except: {left, right}}"""
    if inner := lexed.get("field"):
        return inner
    if inner := lexed.get("and"):
        return {
            "and": {
                "left": clean_as_expr(inner["left"]),
                "right": clean_as_expr(inner["right"]),
            }
        }
    if inner := lexed.get("or"):
        return {
            "or": {
                "left": clean_as_expr(inner["left"]),
                "right": clean_as_expr(inner["right"]),
            }
        }
    if inner := lexed.get("except"):
        return {
            "except": {
                "left": clean_as_expr(inner["left"]),
                "right": clean_as_expr(inner["right"]),
            }
        }


def clean_mp_peering(lexed: dict):
    """-> str | {as_expr: str, [router_expr1]: str, [router_expr2]: str}"""
    if peering_set := lexed.get("peering-set-name"):
        return peering_set
    as_expr_raw = " ".join(lexed["as-expression"])
    if expr := lex_with(as_expr, as_expr_raw):
        result = {"as_expr": clean_as_expr(expr)}
    else:
        return None
    if (expr1 := lexed.get("mp-router-expression-1")) and (
        expr := lex_with(as_expr, " ".join(expr1))
    ):
        result["router_expr1"] = clean_as_expr(expr)
    if (expr2 := lexed.get("mp-router-expression-2")) and (
        expr := lex_with(as_expr, " ".join(expr2))
    ):
        result["router_expr2"] = clean_as_expr(expr)
    return result


def parse_mp_peering(mp_peering_raw: list[str]):
    if lexed := lex_with(mp_peering, " ".join(mp_peering_raw)):
        return clean_mp_peering(lexed)


def parse_import_factor(import_factor_raw: dict):
    import_factor: dict[str, list | dict] = {"mp_peerings": []}
    if filter := lex_with(mp_filter, import_factor_raw["mp-filter"]):
        import_factor["mp_filter"] = clean_mp_filter(filter)
    else:
        return
    for peering_raw in import_factor_raw["mp-peerings"]:
        peering = {}
        if peer := parse_mp_peering(peering_raw["mp-peering"]):
            peering["mp_peering"] = peer
        else:
            continue
        if action_raws := peering_raw.get("actions"):
            peering["actions"] = clean_action(
                action_lexed
                for action_raw in action_raws
                if (action_lexed := lex_with(action, action_raw))
            )
        import_factor["mp_peerings"].append(peering)  # type: ignore
    return import_factor


def parse_afi_import_expression(
    afi_import_expression: dict, afi_entries: list[tuple[str, str]]
) -> list[tuple[list[tuple[str, str]], list[dict]]]:
    """-> list[tuple[afi_entries, parsed]]"""
    if afi_list := afi_import_expression.get("afi-list"):
        afi_entries = merge_afi(
            afi_item for item in afi_list if (afi_item := lex_with(afi, item))
        )

    parsed = []
    if import_factors := afi_import_expression.get("import-factors"):
        for import_factor_raw in import_factors:
            if import_factor := parse_import_factor(import_factor_raw):
                parsed.append(import_factor)
        return [(afi_entries, parsed)]

    if "mp-peerings" in afi_import_expression and "mp-filter" in afi_import_expression:
        if import_factor := parse_import_factor(afi_import_expression):
            parsed.append(import_factor)
        return [(afi_entries, parsed)]

    if "except" in afi_import_expression or "refine" in afi_import_expression:
        # TODO: Handle EXCEPT and REFINE logic.
        print(f"Skipping complex logic in {afi_import_expression}", file=stderr)
        return []

    raise ValueError(f"Illegal keys: {afi_import_expression}.")


def import_export(lexed: dict, result: dict[str, dict[str, list]]):
    """Parse lexed <mp-import> or <mp-export>."""
    if protocol_1 := lexed.get("protocol-1"):
        print(f"Ignoring protocol-1: {protocol_1}.", file=stderr)
    if protocol_2 := lexed.get("protocol-2"):
        print(f"Ignoring protocol-2: {protocol_2}.", file=stderr)

    parsed_list = parse_afi_import_expression(lexed, [("any", "any")])
    for afi_entries, parsed in parsed_list:
        for version, cast in afi_entries:
            version_entry = result.get(version, {})
            cast_entry = version_entry.get(cast, [])
            cast_entry.extend(parsed)
            version_entry[cast] = cast_entry
            result[version] = version_entry
    return result
