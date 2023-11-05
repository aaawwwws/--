use chromiumoxide::{browser::{Browser, BrowserConfig}, cdp::browser_protocol::tethering::BindParams};
use tokio_stream::StreamExt;
struct ScrapingData {
    url:String,
    region:String
}
impl ScrapingData {
    pub fn new (url:&str, region:&str) -> Self {
        Self{url:url.to_string(),region:region.to_string()}
    }
    pub fn get_url (&self) -> &str {
        return &self.url;
    }
    pub fn get_region (&self) -> &str {
        return &self.region;
    }
}



#[tokio::main]
async fn main() -> Result<(),Box<dyn std::error::Error>> {
    //ScrapingData型のインスタンスを生成。
    let data:ScrapingData = ScrapingData::new("https://jp.indeed.com/","北海道");
    let search:&str = data.get_url();
    let region:&str = data.get_region();
    let (browser,mut handler,)= Browser::launch(BrowserConfig::builder().with_head().no_sandbox().build()?).await?;
    let _handle = tokio::task::spawn(async move {
        loop {
            let _ = handler.next().await.unwrap();
        }
    });
    let page = browser.new_page(search).await?;
    let attributes = page.find_element("input.css-1svp6q2.e1jgz0i3").await?.description().await?.attributes.unwrap();
    let region_text = &attributes[17];
    for _i in 0..region_text.len() {
        page.find_element("input.css-1svp6q2.e1jgz0i3").await?.click().await?.press_key("Backspace").await?;
    }
    //binding.click().await?.type_str("aaa").await?;
    //let elements = type_region.find_elements("a.jcs-JobTitle.css-jspxzf.eu4oa1w0").await?;
    //let text = elements.inner_text().await?;
    print!("{:?}",region_text.len());
    Ok(())
}