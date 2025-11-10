/*!
Data structures used to parse the XML stored in a game's TROPCONF.SFM file.
*/

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(rename = "trophyconf")]
pub struct TrophyConf
{
	#[serde(rename = "@version")]
	pub version: String,
	#[serde(rename = "@policy")]
	pub policy: String,
	pub npcommid: String,
	#[serde(rename = "trophyset-version")]
	pub trophysetVersion: String,
	#[serde(rename = "parental-level")]
	pub parentalLevel: ParentalLevel,
	#[serde(rename = "title-name")]
	pub titleName: String,
	#[serde(rename = "title-detail")]
	pub titleDetail: String,
	#[serde(rename = "trophy", default)]
	pub trophies: Vec<TrophyMetadata>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct ParentalLevel
{
	#[serde(rename = "@license-area")]
	pub licenseArea: String,
	#[serde(rename = "#text")]
	pub value: i32,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct TrophyMetadata
{
	#[serde(rename = "@id")]
	pub id: u32,
	#[serde(rename = "@hidden")]
	pub hidden: String,
	#[serde(rename = "@ttype")]
	pub ttype: String,
	#[serde(rename = "@pid")]
	pub pid: i32,
	pub name: String,
	pub detail: String,
}

impl TrophyMetadata
{
	pub const HiddenTrue: &str = "yes";
}

#[cfg(test)]
mod tests
{
	use super::*;
	use crate::rpcs3::TrophyGrade;
	
	const trophyconfXml: &str = r#"<!-- comment -->
<trophyconf version="1.1" policy="large">
	<npcommid>NPXXYYYY_00</npcommid>
	<trophyset-version>01.00</trophyset-version>
	<parental-level license-area="default">0</parental-level>
	<title-name>Game Title</title-name>
	<title-detail>Game Title Detail</title-detail>
	<trophy id="000" hidden="yes" ttype="P" pid="-1">
		<name>Platinum Trophy Name Here</name>
		<detail>Platinum Description text here</detail>
	</trophy>
	<trophy id="001" hidden="no" ttype="B" pid="000">
		<name>Bronze Trophy Name Here</name>
		<detail>Bronze Description text here</detail>
	</trophy>
	<trophy id="002" hidden="no" ttype="S" pid="000">
		<name>Silver Trophy Name Here</name>
		<detail>Silver Description text here</detail>
	</trophy>
	<trophy id="003" hidden="no" ttype="G" pid="000">
		<name>Gold Trophy Name Here</name>
		<detail>Gold Description text here</detail>
	</trophy>
</trophyconf>"#;
	
	const trophyXml: &str = r#"<trophy id="000" hidden="no" ttype="B" pid="000">
	<name>Trophy Name Here</name>
	<detail>Description text here</detail>
</trophy>"#;
	
	#[test]
	fn parseGame()
	{
		let result = serde_xml_rs::from_str::<TrophyConf>(trophyconfXml);
		assert!(result.is_ok());
		
		let conf = result.unwrap();
		assert_eq!(conf.npcommid, "NPXXYYYY_00".to_string());
		assert_eq!(conf.parentalLevel.licenseArea, "default".to_string());
		assert_eq!(conf.parentalLevel.value, 0);
		assert_eq!(conf.policy, "large".to_string());
		assert_eq!(conf.titleDetail, "Game Title Detail".to_string());
		assert_eq!(conf.titleName, "Game Title".to_string());
		assert_eq!(conf.trophysetVersion, "01.00".to_string());
		assert_eq!(conf.version, "1.1".to_string());
		
		assert_ne!(conf.trophies.len(), 0);
		for (i, trophy) in conf.trophies.iter().enumerate()
		{
			assert_eq!(trophy.id, i as u32);
			assert!(!trophy.ttype.is_empty());
			
			match trophy.ttype.clone().into()
			{
				TrophyGrade::Bronze => assert!(trophy.name.contains("Bronze")),
				TrophyGrade::Gold => assert!(trophy.name.contains("Gold")),
				TrophyGrade::Platinum => assert!(trophy.name.contains("Platinum")),
				TrophyGrade::Silver => assert!(trophy.name.contains("Silver")),
				_ => {},
			}
		}
	}
	
	#[test]
	fn parseTrophy()
	{
		let result = serde_xml_rs::from_str::<TrophyMetadata>(trophyXml);
		assert!(result.is_ok());
		
		let trophy = result.unwrap();
		assert_eq!(trophy.id, 0);
		assert_eq!(trophy.hidden, "no".to_string());
		assert_eq!(trophy.ttype, "B".to_string());
		assert_eq!(trophy.pid, 0);
		assert_eq!(trophy.name, "Trophy Name Here".to_string());
		assert_eq!(trophy.detail, "Description text here".to_string());
	}
}
