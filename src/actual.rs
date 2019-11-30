use std::io::{Write};

use failure::{Error};
use hyper::{Body, Client};
use hyper_openssl::{HttpsConnector};
use log::{debug};
use paste;


const BASE_ACTUAL_URL: &'static str = "https://fias.nalog.ru/Public/Downloads/Actual";


macro_rules! fn_pull {
    ($fn:ident, $path:tt) => {
        paste::item! {
            pub async fn [<pull_ $fn>](w: &mut impl Write) -> std::result::Result<(), Error> {
                let url = format!("{}{}", BASE_ACTUAL_URL, $path);
                let url = url.parse::<hyper::Uri>()?;

                let ssl = HttpsConnector::new()?;
                let client = Client::builder().keep_alive(false).build::<_, Body>(ssl);
                let resp = client.get(url).await?;
                
                debug!("Status: {}", resp.status());
                debug!("Headers: {:#?}", resp.headers());

                let mut body = resp.into_body();

                while let Some(chunk) = body.next().await {
                    let chunk = chunk?;
                    w.write_all(chunk.into_bytes().as_ref())?;
                }

                Ok(())
            }
        }
    }
}
fn_pull!(kladr4_arj, "/base.arj");
fn_pull!(kladr4_7z, "/base.7z");
fn_pull!(fias_full_xml, "/fias_xml.rar");
fn_pull!(fias_delta_xml, "/fias_delta_xml.rar");
fn_pull!(fias_full_dbf, "/fias_dbf.rar");
fn_pull!(fias_delta_dbf, "/fias_delta_dbf.rar");
fn_pull!(version_date, "/VerDate.txt");

pub async fn read_version_date() -> Result<String, Error> {
    let mut buf = Vec::with_capacity(32);
    pull_version_date(&mut buf).await?;

    let version_date = String::from_utf8(buf)?;
    Ok(version_date)
}
