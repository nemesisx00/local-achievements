use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct AppliedFilters
{
	pub tags: Option<ProductUserTag>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct FilteredProductsPage
{
	pub appliedFilters: AppliedFilters,
	pub contentSystemCompatibility: Option<bool>,
	pub hasHiddenProducts: bool,
	pub hiddenUpdatedProductsCount: u64,
	pub moviesCount: u64,
	pub page: u64,
	pub products: Vec<Product>,
	pub productsPerPage: u64,
	pub sortBy: Option<String>,
	pub tags: Vec<ProductUserTag>,
	pub totalPages: u64,
	pub totalProducts: u64,
	pub updatedProductsCount: u64,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct FilteredProductsParameters
{
	pub category: Option<String>,
	pub feature: Option<String>,
	pub hiddenFlag: Option<bool>,
	pub language: Option<String>,
	pub mediaType: Option<u64>,
	pub page: Option<u64>,
	pub search: Option<String>,
	pub sortBy: Option<String>,
	pub system: Option<String>,
	pub tags: Option<String>,
	pub totalPages: Option<u64>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Product
{
	pub availability: ProductAvailability,
	pub category: String,
	pub dlcCount: u64,
	pub id: u64,
	pub image: String,
	pub isBaseProductMissing: bool,
	pub isComingSoon: bool,
	pub isGalaxyCompatible: bool,
	pub isGame: bool,
	pub isHidden: bool,
	pub isHidingDisabled: bool,
	pub isInDevelopment: bool,
	pub isMovie: bool,
	pub isNew: bool,
	pub rating: u64,
	pub releaseDate: Option<ProductReleaseDate>,
	pub slug: String,
	pub tags: Vec<String>,
	pub title: String,
	pub updates: u64,
	pub url: String,
	#[serde(default)]
	pub worksOk: Option<ProductWorksOn>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct ProductAvailability
{
	pub isAvailable: bool,
	pub isAvailableInAccount: bool,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct ProductReleaseDate
{
	pub date: String,
	pub timezone: String,
	pub timezone_type: u64,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct ProductUserTag
{
	pub id: String,
	pub name: String,
	pub productCount: String,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct ProductWorksOn
{
	pub Linux: bool,
	pub Mac: bool,
	pub Windows: bool,
}
