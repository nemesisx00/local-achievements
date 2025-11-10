/*!
Data structures used to parse the binary data stored in a game's TROPUSR.DAT
file.
*/

use std::io::{Cursor, Read};
use anyhow::{anyhow, Context, Result};
use byteorder::{BigEndian, ReadBytesExt};

/// Magic number used to identify valid TROPUSR.DAT files
pub const MagicNumber: u32 = 0x818F54AD;

#[allow(unused)]
#[derive(Clone, Debug, Default)]
pub struct DatFile
{
	pub header: Header,
	pub type4: Vec<EntryType4>,
	pub type6: Vec<EntryType6>,
}

impl DatFile
{
	pub fn readFromCursor(cursor: &mut Cursor<Vec<u8>>) -> Result<Self>
	{
		let header = Header::readFromCursor(cursor)?;
		
		if header.magic != MagicNumber
		{
			return Err(anyhow!(
				"RPCS3 Trophy DAT file magic number mismatch: {} != {}",
				header.magic,
				MagicNumber
			));
		}
		
		let mut tableHeaders = vec![];
		cursor.set_position(Header::Size);
		for _ in 0..header.tableCount
		{
			let tableHeader = TableHeader::readFromCursor(cursor)?;
			tableHeaders.push(tableHeader);
		}
		
		let mut type4 = vec![];
		let mut type6 = vec![];
		
		for tableHeader in tableHeaders
		{
			match tableHeader.r#type
			{
				4 => type4 = EntryType4::readAllEntries(cursor, tableHeader)?,
				6 => type6 = EntryType6::readAllEntries(cursor, tableHeader)?,
				_ => {}
			}
		}
		
		return Ok(Self
		{
			header,
			type4,
			type6,
		});
	}
}

#[derive(Clone, Debug, Default)]
pub struct Header
{
	pub magic: u32,
	pub tableCount: u32,
}

impl Header
{
	pub const Size: u64 = 48;
	
	pub fn readFromCursor(cursor: &mut Cursor<Vec<u8>>) -> Result<Self>
	{
		let magic = cursor.read_u32::<BigEndian>()
			.context("Header magic fail")?;
		let _unknown1 = cursor.read_u32::<BigEndian>()
			.context("Header unknown 1 fail")?;
		let tableCount = cursor.read_u32::<BigEndian>()
			.context("Header table count fail")?;
		let _unknown2 = cursor.read_u32::<BigEndian>()
			.context("Header unknown 2 fail")?;
		
		let mut reserved = [0u8; 32];
		_ = cursor.read_exact(&mut reserved)
			.context("Header reserved fail")?;
		
		return Ok(Self
		{
			magic,
			tableCount,
		});
	}
}

#[allow(unused)]
#[derive(Clone, Debug, Default)]
pub struct TableHeader
{
	pub entryCount: u32,
	pub entrySize: u32,
	pub offset: u64,
	pub r#type: u32,
}

impl TableHeader
{
	#[allow(unused)]
	pub const Size: u64 = 32;
	
	pub fn readFromCursor(cursor: &mut Cursor<Vec<u8>>) -> Result<Self>
	{
		let r#type = cursor.read_u32::<BigEndian>()
			.context("ListHeader type fail")?;
		let entrySize = cursor.read_u32::<BigEndian>()
			.context("ListHeader entrySize fail")?;
		let _unknown1 = cursor.read_u32::<BigEndian>()
			.context("ListHeader unknown 1 fail")?;
		let entryCount = cursor.read_u32::<BigEndian>()
			.context("ListHeader entryCount fail")?;
		let offset = cursor.read_u64::<BigEndian>()
			.context("ListHeader offset fail")?;
		let _reserved = cursor.read_u64::<BigEndian>()
			.context("ListHeader reserved fail")?;
		
		return Ok(Self
		{
			entryCount,
			entrySize,
			offset,
			r#type,
		});
	}
}

#[allow(unused)]
#[derive(Clone, Debug, Default)]
pub struct EntryHeader
{
	pub id: u32,
	pub size: u32,
	pub r#type: u32,
}

impl EntryHeader
{
	#[allow(unused)]
	pub const Size: u64 = 16;
	
	pub fn readFromCursor(cursor: &mut Cursor<Vec<u8>>) -> Result<Self>
	{
		let r#type = cursor.read_u32::<BigEndian>()
			.context("EntryHeader type fail")?;
		let size = cursor.read_u32::<BigEndian>()
			.context("EntryHeader size fail")?;
		let id = cursor.read_u32::<BigEndian>()
			.context("EntryHeader id fail")?;
		let _unknown1 = cursor.read_u32::<BigEndian>()
			.context("EntryHeader unknown 1 fail");
		
		return Ok(Self
		{
			id,
			size,
			r#type,
		});
	}
}

#[allow(unused)]
#[derive(Clone, Debug, Default)]
pub struct EntryType4
{
	pub grade: u32,
	pub header: EntryHeader,
	pub id: u32,
	pub pid: u32,
}

impl EntryType4
{
	#[allow(unused)]
	pub const Size: u64 = 96;
	
	pub fn readAllEntries(cursor: &mut Cursor<Vec<u8>>, header: TableHeader) -> Result<Vec<Self>>
	{
		cursor.set_position(header.offset);
		
		let mut list = vec![];
		
		for i in 0..header.entryCount
		{
			let entry = Self::readFromCursor(cursor)
				.context(format!("Failed to read entry {}", i))?;
			
			list.push(entry);
		}
		
		return Ok(list);
	}
	
	pub fn readFromCursor(cursor: &mut Cursor<Vec<u8>>) -> Result<Self>
	{
		let header = EntryHeader::readFromCursor(cursor)
			.context("EntryType4 header fail")?;
		let id = cursor.read_u32::<BigEndian>()
			.context("EntryType 4 id fail")?;
		let grade = cursor.read_u32::<BigEndian>()
			.context("EntryType4 grade fail")?;
		let pid = cursor.read_u32::<BigEndian>()
			.context("EntryType4 pid fail")?;
		
		return Ok(Self
		{
			grade,
			header,
			id,
			pid,
		});
	}
}

#[allow(unused)]
#[derive(Clone, Debug, Default)]
pub struct EntryType6
{
	pub header: EntryHeader,
	pub timestamp1: u64,
	pub timestamp2: u64,
	pub trophyId: u32,
	pub trophyState: u32,
}

impl EntryType6
{
	#[allow(unused)]
	pub const Size: u64 = 112;
	
	pub fn readAllEntries(cursor: &mut Cursor<Vec<u8>>, header: TableHeader) -> Result<Vec<Self>>
	{
		cursor.set_position(header.offset);
		
		let mut list = vec![];
		
		for i in 0..header.entryCount
		{
			let entry = Self::readFromCursor(cursor)
				.context(format!("Failed to read entry {}", i))?;
			
			list.push(entry);
		}
		
		return Ok(list);
	}
	
	pub fn readFromCursor(cursor: &mut Cursor<Vec<u8>>) -> Result<Self>
	{
		let header = EntryHeader::readFromCursor(cursor)
			.context("EntryType6 header fail")?;
		let trophyId = cursor.read_u32::<BigEndian>()
			.context("EntryType6 trophyId fail")?;
		let trophyState = cursor.read_u32::<BigEndian>()
			.context("EntryType6 trophyState fail")?;
		let _unknown1 = cursor.read_u32::<BigEndian>()
			.context("EntryType6 unknown 1 fail")?;
		let _unknown2 = cursor.read_u32::<BigEndian>()
			.context("EntryType6 unknown 2 fail")?;
		let timestamp1 = cursor.read_u64::<BigEndian>()
			.context("EntryType6 timestamp1 fail")?;
		let timestamp2 = cursor.read_u64::<BigEndian>()
			.context("EntryType6 timestamp2 fail")?;
		
		let mut _unknown3 = [0u8; 64];
		_ = cursor.read_exact(&mut _unknown3)
			.context("EntryType6 unknown 3 fail")?;
		
		return Ok(Self
		{
			header,
			timestamp1,
			timestamp2,
			trophyId,
			trophyState,
		});
	}
}

#[cfg(test)]
mod tests
{
	use std::env;
	use std::path::Path;
	use anyhow::Result;
	use super::*;
	
	fn readTrophyFile() -> Result<Cursor<Vec<u8>>>
	{
		let datPath = env::var("RPCS3_TEST_TROPUSR_PATH")?;
		let path = Path::new(&datPath);
		let buffer = std::fs::read(path)?;
		return Ok(Cursor::new(buffer));
	}
	
	/**
	Requires an environment variable to be set in order to run successfully.
	
	- `RPCS3_TEST_TROPUSR_PATH`: The absolute path to a TROPUSR.DAT file.
	*/
	#[ignore]
	#[test]
	fn parseTrophyUsr()
	{
		let cursorResult = readTrophyFile();
		assert!(cursorResult.is_ok());
		let mut cursor = cursorResult.unwrap();
		
		let header = Header::readFromCursor(&mut cursor).unwrap();
		assert_eq!(header.magic, MagicNumber);
		assert_eq!(header.tableCount, 2);
		
		let mut tables = vec![];
		cursor.set_position(Header::Size);
		for _ in 0..header.tableCount
		{
			let tableHeader = TableHeader::readFromCursor(&mut cursor).unwrap();
			tables.push(tableHeader);
		}
		
		assert!(tables.iter().any(|th| th.r#type == 4));
		assert!(tables.iter().any(|th| th.r#type == 6));
		
		for tableHeader in tables
		{
			assert!(tableHeader.r#type == 4 || tableHeader.r#type == 6);
			let entryCount = tableHeader.entryCount;
			match tableHeader.r#type
			{
				4 => {
					let type4Entries = EntryType4::readAllEntries(&mut cursor, tableHeader).unwrap();
					assert_eq!(type4Entries.len() as u32, entryCount);
				},
				
				6 => {
					let type6Entries = EntryType6::readAllEntries(&mut cursor, tableHeader).unwrap();
					assert_eq!(type6Entries.len() as u32, entryCount);
				},
				
				_ => {}
			}
		}
	}
}
