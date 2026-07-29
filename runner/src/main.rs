use anyhow::Context as _;
use china_unicom_oxidebot::ChinaUnicomPlugin;
use oxidebot::prelude::*;
use oxidebot_adapter_telegram::TelegramAdapter;

#[derive(BotState)]
struct AppState {
    #[state]
    china_unicom: ChinaUnicomPlugin,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let token = std::env::var("TELEGRAM_BOT_TOKEN")
        .context("TELEGRAM_BOT_TOKEN is not set; configure it in env.conf")?;
    let bot_id = std::env::var("TELEGRAM_BOT_ID").context(
        "TELEGRAM_BOT_ID is not set; use the stable numeric ID before the colon in the bot token",
    )?;
    let china_unicom = ChinaUnicomPlugin::open().await?;

    OxideBot::with_state(AppState {
        china_unicom: china_unicom.clone(),
    })
    .adapter(TelegramAdapter::new(token, bot_id)?)
    .plugin(china_unicom.bundle::<AppState>())
    .include(Module::new().help())
    .run()
    .await?;

    Ok(())
}
