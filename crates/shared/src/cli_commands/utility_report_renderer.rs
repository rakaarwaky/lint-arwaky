// PURPOSE: utility_report_renderer — print the summary table and its deltas.
// Text only; the scan report's own renderers own the per-violation output.
use crate::taxonomy_report_snapshot_vo::ReportDelta;
use crate::taxonomy_report_snapshot_vo::ReportSnapshot;

/// How many members to name before collapsing the tail into one line.
///
/// A forty-member workspace does not fit a terminal, and the members that
/// cleared entirely are the least interesting part of it.
const MAX_ROWS: usize = 20;

/// Print the report: counts per member, and what changed since the last run.
pub fn render_report(target: &str, delta: &ReportDelta, current: &ReportSnapshot) {
    println!("Lint Arwaky — Progress Report");
    println!("Target: {target}");
    println!("Taken:  {}", format_time(current.taken_at));
    println!();

    let member_width = delta
        .members
        .iter()
        .map(|m| m.member.len())
        .max()
        .unwrap_or(6)
        .min(40);

    println!(
        "{:<member_width$}  {:>9}  {:>7}",
        "Member", "Violations", "Delta"
    );
    println!("{}", "─".repeat(member_width + 21));

    for (index, m) in delta.members.iter().enumerate() {
        if index == MAX_ROWS && delta.members.len() > MAX_ROWS {
            let hidden = delta.members.len() - MAX_ROWS;
            println!("… and {hidden} more member{}", plural(hidden));
            break;
        }
        println!(
            "{:<member_width$}  {:>9}  {:>7}",
            truncate(&m.member, member_width),
            m.now,
            change_text(m.change())
        );
    }

    println!("{}", "─".repeat(member_width + 21));
    println!(
        "{:<member_width$}  {:>9}  {:>7}",
        "Total",
        delta.total_now,
        change_text(delta.total_change())
    );
    println!();

    match delta.since {
        Some(since) => println!(
            "Fixed {} · Introduced {} · since {}",
            delta.fixed(),
            delta.introduced(),
            format_time(since)
        ),
        None => println!(
            "No baseline yet — this scan is the first one recorded for this target, \
             so it shows absolute counts only. Run it again to see what changed."
        ),
    }
    println!();
    println!("  lint-arwaky-cli scan <path> — see each violation with its reason and remedy");
    println!("  lint-arwaky-cli skill list — pick the skill that matches the file you are fixing");
}

/// A change as a reader reads it: worse, better, or unchanged.
///
/// A sign alone would do, but "−7" next to "38" is ambiguous without knowing
/// which direction is good, and an arrow costs three characters.
fn change_text(change: Option<i64>) -> String {
    match change {
        None => "—".to_string(),
        Some(0) => "·".to_string(),
        Some(n) if n > 0 => format!("▲ +{n}"),
        // The arrow already points down, so the minus would read as twice.
        Some(n) => format!("▼ {}", n.abs()),
    }
}

/// Seconds since the epoch as `YYYY-MM-DD HH:MM` in UTC.
///
/// Hand-rolled from the civil calendar rather than pulling in a date library:
/// the conversion from days to a date is the whole of it, and the report has
/// no other date to print.
fn format_time(seconds: i64) -> String {
    if seconds <= 0 {
        return "unknown".to_string();
    }
    let days = seconds.div_euclid(86_400);
    let time_of_day = seconds.rem_euclid(86_400);
    let (hour, minute) = (time_of_day / 3600, (time_of_day % 3600) / 60);

    // Days since 1970-01-01 to a civil date, via the standard era algorithm.
    let z = days + 719_468;
    let era = z.div_euclid(146_097);
    let day_of_era = z.rem_euclid(146_097);
    let year_of_era =
        (day_of_era - day_of_era / 1460 + day_of_era / 36_524 - day_of_era / 146_096) / 365;
    let year = year_of_era + era * 400;
    let day_of_year = day_of_era - (365 * year_of_era + year_of_era / 4 - year_of_era / 100);
    let mp = (5 * day_of_year + 2) / 153;
    let day = day_of_year - (153 * mp + 2) / 5 + 1;
    let month = if mp < 10 { mp + 3 } else { mp - 9 };
    let year = if month <= 2 { year + 1 } else { year };

    format!("{year:04}-{month:02}-{day:02} {hour:02}:{minute:02}")
}

fn truncate(text: &str, width: usize) -> String {
    if text.len() <= width {
        return text.to_string();
    }
    let mut out: String = text.chars().take(width.saturating_sub(1)).collect();
    out.push('…');
    out
}

fn plural(count: usize) -> &'static str {
    if count == 1 { "" } else { "s" }
}
