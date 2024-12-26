use indicatif::ProgressStyle;

pub fn progress_style() -> ProgressStyle {
    ProgressStyle::with_template(
        "[Elapsed: {elapsed_precise} | ETA: {eta_precise}] {wide_bar} {human_pos}/{human_len} ",
    )
    .unwrap()
}
