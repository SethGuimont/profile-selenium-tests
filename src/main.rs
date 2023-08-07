use thirtyfour::prelude::*;

#[tokio::main]
async fn main() -> WebDriverResult<()> {
    let caps = DesiredCapabilities::chrome();
    let driver = WebDriver::new("http://localhost:9515", caps).await?;

    // Navigate to my profile page
    driver.goto("https://sethguimont.github.io").await?;
    assert_eq!(driver.title().await?, "Seth's Portfolio Page - Home");

    // Always explicitly close the browser.
    driver.quit().await?;

    Ok(())
}
