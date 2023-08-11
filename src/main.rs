use thirtyfour::prelude::*;

#[tokio::main]
async fn main() -> WebDriverResult<()> {
    let mut caps = DesiredCapabilities::chrome();
    caps.add_chrome_arg("--enable-automation")?;
    let driver = WebDriver::new("http://localhost:9515", caps).await?;

    // Navigate to my profile page
    driver.goto("https://sethguimont.github.io").await?;
    assert_eq!(driver.title().await?, "Seth's Portfolio Page - Home");
    println!("Title = {}", driver.title().await?);

    driver.find(By::Tag("header-component")).await?;
    driver.find(By::Tag("footer-component")).await?;

    // Navigate to projects page
    driver.goto("https://sethguimont.github.io/projects.html").await?;
    println!("Title = {}", driver.title().await?);
    driver.find(By::Tag("header-component")).await?;
    driver.find(By::Tag("footer-component")).await?;

    // Navigate to Blog
    driver.goto("https://sethguimont.github.io/blog.html").await?;
    println!("Title = {}", driver.title().await?);
    driver.find(By::Tag("header-component")).await?;
    driver.find(By::Tag("footer-component")).await?;

    //Navigate to Contact
    driver.goto("https://sethguimont.github.io/contact.html").await?;
    println!("Title = {}", driver.title().await?);
    driver.find(By::Tag("header-component")).await?;
    driver.find(By::Tag("footer-component")).await?;

    // Navigate to Discord
    driver.goto("https://sethguimont.github.io/discord.html").await?;
    println!("Title = {}", driver.title().await?);
    driver.find(By::Tag("header-component")).await?;
    driver.find(By::Tag("footer-component")).await?;

    // Always explicitly close the browser.
    driver.quit().await?;

    Ok(())
}
