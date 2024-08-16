# Artifacts

Follow the instructions below to reproduce the artifacts.

## Setup

1. Get yourself a UNIX environment. Try WSL if you are on Windows.

1. Make sure you have these CLI tools:

    ```text
    rg
    ```

1. Make sure you have Python 3.11+ and PyPy3.11+, and they are on path.
    You may create virtual environments as you like.

1. Clone and enter this repository:

    ```sh
    git clone --depth=1 https://github.com/SichangHe/internet_route_verification.git
    cd internet_route_verification
    ```

1. Download the source data from [Raw data used, for
    reproducibility](https://github.com/SichangHe/internet_route_verification/releases/tag/raw-data)
    and follow the instruction there to unpack them to
    the correct directory structure.

## Results to reproduce

- [ ] abstract & intro & sec 5 & appendix B Limitations:
    parse and interpret 99.99% of RPSL policies.
    We leave the handling of 60 rules whose filters contain AS-path regex with
    ASN ranges (21 rules) or samepattern unary postfix operators (e.g., ~*,
    39 rules) as future work.
    we ignore 54 rules with BGP community attributes in their filters.
    <https://github.com/SichangHe/internet_route_verification/issues/106>
- [ ] intro: 53.2% of ASes not declaring any policies.
    <https://github.com/SichangHe/internet_route_verification/issues/161>
- [ ] intro:
    a large portion of interconnections present in BGP routes (40.4%)
    cannot be verified using the RPSL due to missing information.
    <https://github.com/SichangHe/internet_route_verification/issues/162>
- [ ] intro: For interconnections covered in the RPSL,
    we observe a high fraction (29.3%) of strict matches.
    We explain most mismatches (19.0%)
    by
    six common mistakes we identified
    <https://github.com/SichangHe/internet_route_verification/issues/162>
- [ ] sec 3: RPSLyzer parses the 13 IRRs listed in Table 1,
    totaling 7.1 GiB of data, and exports the IR,
    all in under five minutes on an Apple M1
- [ ] sec 4: Table 1.
    <https://github.com/SichangHe/internet_route_verification/issues/126>.

    ```sh
    ls -l | awk 'BEGIN { printf "%-50s %10s MiB\n", "File", "Size" } NR>1 { size=$5/1024/1024; printf "%-50s %10.3f MiB\n", $9, size }'
    ```

    35.4% of aut-nums contain no rules,
    <https://github.com/SichangHe/internet_route_verification/issues/60>,
    10.9% define at least 10 rules, and 0.13% (101 aut-nums)
    define over 1000 rules.
    <https://github.com/SichangHe/internet_route_verification/issues/122>
- [ ] sec 4:
    no significant correlation between how many rules an AS defines and
    how many neighbors, customers, peers,
    or providers it has in CAIDA’s AS-relationship database.
    <https://github.com/SichangHe/internet_route_verification/issues/19>.
    <https://github.com/SichangHe/internet_route_verification/issues/95>.
    <https://github.com/SichangHe/internet_route_verification/issues/109>
- [ ] sec 4: Almost all (98.1%)
    peering definitions comprise a single ASN or ANY.
    <https://github.com/SichangHe/internet_route_verification/issues/107>.
    <https://github.com/SichangHe/internet_route_verification/issues/64>
- [ ] sec 4: Most (95.0%)
    ASes with rules only specify simple filters compatible with BGPq4.
    <https://github.com/SichangHe/internet_route_verification/issues/64>
- [ ] sec 4:
    Table 2 shows that 60.4% of aut-num and 31.7% of
    as-set objects are referenced in filter definitions.
    <https://github.com/SichangHe/internet_route_verification/issues/123>
- [ ] sec 4: most filters are either an as-set (43.4%) or ASN (24.1%).
    <https://github.com/SichangHe/internet_route_verification/issues/159>
- [ ] sec 4: Our IRR dumps contain 3,904,352 route objects,
    corresponding to 3,367,914 unique prefix-origin pairs and
    2,817,344 unique prefixes.
    697,269 (24.7%) have multiple route objects defined,
    among which 404,901 (58.1%)
    prefixes have route objects with different origins.
    Furthermore, 469,003 (67.3%)
    prefixes have route objects defined by multiple operators.
    <https://github.com/SichangHe/internet_route_verification/issues/138>
- [ ] sec 4: Among 53,268 as-set objects across all IRRs, 7754 (14.6%)
    have no members. 17,434 (32.7%) as-sets contain only one member AS.
    A few (772, 1.4%) extremely large as-sets have more than 10,000 members.
    <https://github.com/SichangHe/internet_route_verification/issues/114>.
    We find that 13,602 (25.5%) of as-sets recursively contain other as-sets,
    among which 3050 (22.4%) form loops and 3129 (23.0%) have depth 5 or more.
    <https://github.com/SichangHe/internet_route_verification/issues/114#issuecomment-1903153622>
- [ ] sec 4: RPSLyzer found 663 syntax errors, 12 invalid as-set names,
    and 17 invalid route-set names.
    <https://github.com/SichangHe/internet_route_verification/issues/57>
- [ ] sec 4: Common syntax errors include out-of-place text,
    such as broken comma-separated lists, misplaced comments,
    invalid RPSL keywords in import and export rules, or plain typos.
    <https://github.com/SichangHe/internet_route_verification/discussions/39>
- [ ] sec 5:
    We ignore 0.06% of routes that are direct exports from
    the collector’s peer ASes and 0.03% of
    routes whose AS-paths contain BGP AS-sets.
    <https://github.com/SichangHe/internet_route_verification/issues/111>
- [ ] sec 5:
    Verifying the 779.3 million routes in all 60 BGP dumps took 2 h 49 m and
    less than 2 GiB of RAM.
    <https://github.com/SichangHe/internet_route_verification/issues/157>
- [ ] sec 5.1: More than half (6664, 64.4%)
    of transit ASes specify themselves as an export rule’s filter.
    <https://github.com/SichangHe/internet_route_verification/issues/134>
- [ ] sec 5.1: 3090 (29.8%)
    transit ASes specify a customer AS C in both an import rule’s peering and
    filter.
    <https://github.com/SichangHe/internet_route_verification/issues/134>
- [ ] sec 5.1: A few (46, 0.44%)
    transit ASes only specify rules for their providers.
    <https://github.com/SichangHe/internet_route_verification/issues/134>
- [ ] sec 5.1: A few (46, 0.44%)
    transit ASes only specify rules for their providers.
    <https://github.com/SichangHe/internet_route_verification/issues/134>
- [ ] sec 5.2: Figure 2
- [ ] sec 5.2: The majority (61,725, 74.4%)
    of ASes have all imports and exports with identical statuses.
    We identified 14.2% of ASes with 100% of propagation verified (yellow),
    51.6% lacking RPSL information (“unrecorded”, green),
    0.34% that only use relaxed filters (blue),
    and 6.9% with only safelisted relationships (red).
    <https://github.com/SichangHe/internet_route_verification/issues/90>
- [ ] sec 5.2: ASes with skipped verifications only constitute 0.03% of ASes.
    <https://github.com/SichangHe/internet_route_verification/issues/99#issuecomment-2085134606>
- [ ] sec 5.2: Out of the 54.9% of ASes with unrecorded cases,
    most can be explained by 27.2% of ASes missing aut-num objects and 24.2% of
    aut-nums with no rules.
    Excluding ASes with skipped or unrecorded cases,
    we find more ASes with verified (76.3%) or special-cased (62.5%)
    routes than ASes with unverified routes (23.1%).
    <https://github.com/SichangHe/internet_route_verification/issues/154>
- [ ] sec 5.2:
    25,596 ASes with at least one special-cased import or export
    (30.9% out of all ASes).
    Among these ASes, more incorrectly allow customer route exports (994,
    “export self”) than imports (325, “import customer”).
    <https://github.com/SichangHe/internet_route_verification/issues/99#issuecomment-2094205769>
- [ ] sec 5.2:
    most of the special cases are due to uphill propagation with
    no matching rules (23,298 ASes) or missing route objects (5181 ASes).
    <https://github.com/SichangHe/internet_route_verification/issues/78>
- [ ] sec 5.2: Figure 3.
- [ ] sec 5.2: For imports,
    we find 96% of AS pairs have a single consistent status;
    this number is 92% for exports.
    <https://github.com/SichangHe/internet_route_verification/issues/96>
- [ ] sec 5.2: over half of AS pairs have unverified routes (418,328, 63.0%).
    <https://github.com/SichangHe/internet_route_verification/issues/94#issuecomment-1822005650>.
    most of them (98.98%)
    fail verification because the relationship is not declared in the RPSL.
    <https://github.com/SichangHe/internet_route_verification/issues/117>
- [ ] sec 5.2: Figure 4
- [ ] sec 5.2: Only 6.6% of routes have the same status across all hops,
    captured by having a bar of single color (1.6% verified, 3.0% unrecorded,
    and 1.6% unverified).
    <https://github.com/SichangHe/internet_route_verification/issues/99#issuecomment-2085328442>.
    <https://github.com/SichangHe/internet_route_verification/issues/88>.
    <https://github.com/SichangHe/internet_route_verification/issues/38#issuecomment-1725626125>
- [ ] sec 5.2:
    We also assess the verification status of the first hop in ASpaths…
    Unfortunately, the results are similar (not shown).
    <https://github.com/SichangHe/internet_route_verification/issues/141>
- [ ] appendix B:
    two cases of non-standard but common syntax used by operators (4724 times…)
- [ ] appendix C:
    <https://github.com/SichangHe/internet_route_verification/issues/83>
- [ ] appendix D: Figure 5.
- [ ] appendix D:
    the most common unrecorded case is 22,562 ASes not having an aut-num
    object.
    The second most common type is for 20,048 ASes that have zero import
    (or export) rules when verifying an import (or export).
- [ ] appendix D:
    Fewer ASes have rules that refer to ASes with
    no originating route objects (zero-route ASes, 2706),
    or set objects (as-set, route-set, peering-set, and filter-set)
    missing in the IRRs (414).
- [ ] appendix D: Figure 6
- [ ] appendix D: A small portion (325, 0.4%) of ASes use “import customer”,
    while more (994, 1.2%) use “export self”.
    <https://github.com/SichangHe/internet_route_verification/issues/99#issuecomment-2094205769>.
    A significant portion (6.2%) of ASes have missing route objects.
    ASes that have uphill propagation with
    no matching RPSL rules occupy a large 28.1% of all ASes,
    much more than the 12.4% of ASes with unverified routes.
    <https://github.com/SichangHe/internet_route_verification/issues/78>