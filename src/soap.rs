use std::io::{Write};

use failure::{Error};
use hyper::{Body, Client, Request};
use hyper_openssl::{HttpsConnector};


const SERVICE_URI: &str = "https://fias.nalog.ru/WebServices/Public/DownloadService.asmx";

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DownloadFileInfo {
    version_id: u64,
    text_version: Option<String>,
    fias_complete_dbf_url: Option<String>,
    fias_complete_xml_url: Option<String>,
    fias_delta_dbf_url: Option<String>,
    fias_delta_xml_url: Option<String>,
	kladr4_arj_url: Option<String>,
    kladr4_7z_url: Option<String>,
}

pub struct DownloadService {
    client: Client,
}

impl DownloadService {
    
    pub fn new() -> Result<Self, Error> {
        let uri = SERVICE_URI.parse::<hyper::Uri>()?;
        let ssl = HttpsConnector::new()?;
        let client = Client::builder().keep_alive(false).build::<_, Body>(ssl);
        Ok(Self { uri, client })
    }

    pub async fn get_last_downlad_file_info(&self) -> Result<DownloadFileInfo, Error> {
        let req = Request::builder()
            .method("POST")
            .uri(SERVICE_URI)
            .body(Body::from(last::REQUEST))?
        let resp = self.client.request(req).await?;
        let envelope: last::Envelope = quick_xml::de::from_reader(resp.into_body())?;
        Ok(envelope.body.resp.result.0)
    }

    pub async fn get_all_download_file_info(&self) -> Result<Vec<DownloadFileInfo>, Error> {
        let req = Request::builder()
            .method("POST")
            .uri(SERVICE_URI)
            .body(Body::from(all::REQUEST))?
        let resp = self.client.request(req).await?;
        let envelope: all::Envelope = quick_xml::de::from_reader(resp.into_body())?;
        Ok(envelope.body.resp.result.0)
    }
}


mod all {
    use super::DownloadFileInfo;

    const REQUEST: &str = r#"<?xml version="1.0" encoding="UTF-8"?>
    <SOAP-ENV:Envelope xmlns:SOAP-ENV="http://schemas.xmlsoap.org/soap/envelope/" xmlns:ns1="https://fias.nalog.ru/WebServices/Public/DownloadService.asmx/">
      <SOAP-ENV:Body>
        <ns1:GetAllDownloadFileInfo/>
      </SOAP-ENV:Body>
    </SOAP-ENV:Envelope>
    "#;

    #[derive(Debug, Deserialize)]
    #[serde(rename_all = "PascalCase")]
    pub struct Envelope {
        body: Body
    }

    #[derive(Debug, Deserialize)]
    #[serde(rename_all = "PascalCase")]
    pub struct Body {
        #[serde(alias = "GetLastDownloadFileInfoResponse")]
        resp: GetAllDownloadFileInfoResponse, 
    }

    #[derive(Debug, Deserialize)]
    #[serde(rename_all = "PascalCase")]
    pub struct GetLastDownloadFileInfoResponse {
        #[serde(alias = "GetLastDownloadFileInfoResult")]
        result: GetAllDownloadFileInfoResult, 
    }

    #[derive(Debug, Deserialize)]
    #[serde(transparent)]
    pub struct GetAllDownloadFileInfoResult(Vec<DownloadFileInfo>);
}

mod last {
    use super::DownloadFileInfo;

    const REQUEST: &str = r#"<?xml version="1.0" encoding="UTF-8"?>
    <SOAP-ENV:Envelope xmlns:SOAP-ENV="http://schemas.xmlsoap.org/soap/envelope/" xmlns:ns1="https://fias.nalog.ru/WebServices/Public/DownloadService.asmx/">
      <SOAP-ENV:Body>
        <ns1:GetLastDownloadFileInfo/>
      </SOAP-ENV:Body>
    </SOAP-ENV:Envelope>
    "#;

    #[derive(Debug, Deserialize)]
    #[serde(rename_all = "PascalCase")]
    pub struct Envelope {
        body: Body
    }

    #[derive(Debug, Deserialize)]
    #[serde(rename_all = "PascalCase")]
    pub struct Body {
        #[serde(alias = "GetLastDownloadFileInfoResponse")]
        resp: GetLastDownloadFileInfoResponse, 
    }

    #[derive(Debug, Deserialize)]
    #[serde(rename_all = "PascalCase")]
    pub struct GetLastDownloadFileInfoResponse {
        #[serde(alias = "GetLastDownloadFileInfoResult")]
        result: GetLastDownloadFileInfoResult, 
    }

    #[derive(Debug, Deserialize)]
    #[serde(transparent)]
    pub struct GetLastDownloadFileInfoResult(DownloadFileInfo);
}
