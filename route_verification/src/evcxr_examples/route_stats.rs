use super::*;

/// Generate statistics for routes.
/// Copy this after running code from [`parse_bgp_lines`].
fn gen_route_stats(query: QueryIr, mut bgp_lines: Vec<Line>, db: AsRelDb) -> Result<()> {
    let start = Instant::now();
    let stats: Vec<RouteStats> = bgp_lines
        .par_iter_mut()
        .map(|line| line.compare.route_stats(&query, &db))
        .collect();
    let size = stats.len();
    println!(
        "Generated stats of {size} routes in {}ms.",
        start.elapsed().as_millis()
    );

    let mut file = BufWriter::new(File::create("route_stats.csv")?);
    file.write_all(RouteStats::csv_header().trim_end_matches(',').as_bytes());
    file.write_all(b"\n");
    let comma = b","[0];
    for s in stats {
        file.write_all(&s.as_csv_bytes());
        file.write_all(b"\n");
    }
    file.flush()?;
    drop(file);

    Ok(())
}
