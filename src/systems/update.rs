use crate::prelude::*;

pub fn update_scoreboard(
    score: Res<Score>,
    score_root: Query<
        Entity,
        (
            With<ScoreboardUi>,
            With<Text>,
        ),
    >,
    mut writer: TextUiWriter,
) {
    if let Ok(score_root) = score_root.single() {
        *writer.text(
            score_root, 1,
        ) = score[0].to_string();
        *writer.text(
            score_root, 2,
        ) = format!(
            " - {}",
            score[1].to_string()
        );
    }
}
