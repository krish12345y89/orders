use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize)]
pub struct Orders {
    #[serde(rename = "OrderId")]
    pub order_id: Uuid,
    
    #[serde(rename = "NumOrderId")]
    pub num_order_id: u32,
    
    #[serde(rename = "Processed")]
    pub processed: bool,
    
    #[serde(rename = "ProcessedDateTime")]
    pub processed_date_time: DateTime<Utc>,
    
    #[serde(rename = "FulfilmentLocationId")]
    pub fulfilment_location_id: Uuid,
    
    #[serde(rename = "GeneralInfo")]
    pub general_info: GeneralInfo,
    
    #[serde(rename = "ShippingInfo")]
    pub shipping_info: ShippingInfo,
    
    #[serde(rename = "CustomerInfo")]
    pub customer_info: CustomerInfo,
    
    #[serde(rename = "TotalsInfo")]
    pub totals_info: TotalsInfo,
    
    #[serde(rename = "ExtendedProperties")]
    pub extended_properties: Vec<ExtendedProperty>,
    
    #[serde(rename = "FolderName")]
    pub folder_name: Vec<String>,
    
    #[serde(rename = "Items")]
    pub items: Vec<Item>,
    
    #[serde(rename = "Notes")]
    pub notes: Vec<Note>,
    
    #[serde(rename = "PaidDateTime")]
    pub paid_date_time: DateTime<Utc>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GeneralInfo {
    #[serde(rename = "Status")]
    pub status: i32,
    
    #[serde(rename = "LabelPrinted")]
    pub label_printed: bool,
    
    #[serde(rename = "LabelError")]
    pub label_error: String,
    
    #[serde(rename = "InvoicePrinted")]
    pub invoice_printed: bool,
    
    #[serde(rename = "PickListPrinted")]
    pub pick_list_printed: bool,
    
    #[serde(rename = "IsRuleRun")]
    pub is_rule_run: bool,
    
    #[serde(rename = "Notes")]
    pub notes: i32,
    
    #[serde(rename = "PartShipped")]
    pub part_shipped: bool,
    
    #[serde(rename = "Marker")]
    pub marker: i32,
    
    #[serde(rename = "IsParked")]
    pub is_parked: bool,
    
    #[serde(rename = "ReferenceNum")]
    pub reference_num: String,
    
    #[serde(rename = "SecondaryReference")]
    pub secondary_reference: String,
    
    #[serde(rename = "ExternalReferenceNum")]
    pub external_reference_num: String,
    
    #[serde(rename = "ReceivedDate")]
    pub received_date: DateTime<Utc>,
    
    #[serde(rename = "Source")]
    pub source: String,
    
    #[serde(rename = "SubSource")]
    pub sub_source: String,
    
    #[serde(rename = "HoldOrCancel")]
    pub hold_or_cancel: bool,
    
    #[serde(rename = "DespatchByDate")]
    pub despatch_by_date: DateTime<Utc>,
    
    #[serde(rename = "HasScheduledDelivery")]
    pub has_scheduled_delivery: bool,
    
    #[serde(rename = "Location")]
    pub location: Uuid,
    
    #[serde(rename = "NumItems")]
    pub num_items: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ShippingInfo {
    #[serde(rename = "Vendor")]
    pub vendor: String,
    
    #[serde(rename = "PostalServiceId")]
    pub postal_service_id: Uuid,
    
    #[serde(rename = "PostalServiceName")]
    pub postal_service_name: String,
    
    #[serde(rename = "TotalWeight")]
    pub total_weight: f64,
    
    #[serde(rename = "ItemWeight")]
    pub item_weight: f64,
    
    #[serde(rename = "PackageCategoryId")]
    pub package_category_id: Uuid,
    
    #[serde(rename = "PackageCategory")]
    pub package_category: String,
    
    #[serde(rename = "PackageTypeId")]
    pub package_type_id: Uuid,
    
    #[serde(rename = "PackageType")]
    pub package_type: String,
    
    #[serde(rename = "PostageCost")]
    pub postage_cost: f64,
    
    #[serde(rename = "PostageCostExTax")]
    pub postage_cost_ex_tax: f64,
    
    #[serde(rename = "TrackingNumber")]
    pub tracking_number: String,
    
    #[serde(rename = "ManualAdjust")]
    pub manual_adjust: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Address {
    #[serde(rename = "EmailAddress")]
    pub email_address: String,
    
    #[serde(rename = "Address1")]
    pub address1: String,
    
    #[serde(rename = "Address2")]
    pub address2: String,
    
    #[serde(rename = "Address3")]
    pub address3: String,
    
    #[serde(rename = "Town")]
    pub town: String,
    
    #[serde(rename = "Region")]
    pub region: String,
    
    #[serde(rename = "PostCode")]
    pub post_code: String,
    
    #[serde(rename = "Country")]
    pub country: String,
    
    #[serde(rename = "FullName")]
    pub full_name: String,
    
    #[serde(rename = "Company")]
    pub company: String,
    
    #[serde(rename = "PhoneNumber")]
    pub phone_number: String,
    
    #[serde(rename = "CountryId")]
    pub country_id: Uuid,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CustomerInfo {
    #[serde(rename = "ChannelBuyerName")]
    pub channel_buyer_name: String,
    
    #[serde(rename = "Address")]
    pub address: Address,
    
    #[serde(rename = "BillingAddress")]
    pub billing_address: Address,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TotalsInfo {
    #[serde(rename = "Subtotal")]
    pub subtotal: f64,
    
    #[serde(rename = "PostageCost")]
    pub postage_cost: f64,
    
    #[serde(rename = "PostageCostExTax")]
    pub postage_cost_ex_tax: f64,
    
    #[serde(rename = "Tax")]
    pub tax: f64,
    
    #[serde(rename = "TotalCharge")]
    pub total_charge: f64,
    
    #[serde(rename = "PaymentMethod")]
    pub payment_method: String,
    
    #[serde(rename = "PaymentMethodId")]
    pub payment_method_id: Uuid,
    
    #[serde(rename = "ProfitMargin")]
    pub profit_margin: f64,
    
    #[serde(rename = "TotalDiscount")]
    pub total_discount: f64,
    
    #[serde(rename = "Currency")]
    pub currency: String,
    
    #[serde(rename = "CountryTaxRate")]
    pub country_tax_rate: f64,
    
    #[serde(rename = "ConversionRate")]
    pub conversion_rate: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtendedProperty {
    #[serde(rename = "RowId")]
    pub row_id: Uuid,
    
    #[serde(rename = "Name")]
    pub name: String,
    
    #[serde(rename = "Value")]
    pub value: String,
    
    #[serde(rename = "Type")]
    pub property_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BinRack {
    #[serde(rename = "Quantity")]
    pub quantity: i32,
    
    #[serde(rename = "BinRack")]
    pub bin_rack: String,
    
    #[serde(rename = "Location")]
    pub location: Uuid,
    
    #[serde(rename = "BatchId")]
    pub batch_id: Option<i32>,
    
    #[serde(rename = "OrderItemBatchId")]
    pub order_item_batch_id: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Item {
    #[serde(rename = "ItemId")]
    pub item_id: Uuid,
    
    #[serde(rename = "ItemNumber")]
    pub item_number: String,
    
    #[serde(rename = "SKU")]
    pub sku: String,
    
    #[serde(rename = "ItemSource")]
    pub item_source: String,
    
    #[serde(rename = "Title")]
    pub title: String,
    
    #[serde(rename = "Quantity")]
    pub quantity: i32,
    
    #[serde(rename = "CategoryName")]
    pub category_name: String,
    
    #[serde(rename = "StockLevelsSpecified")]
    pub stock_levels_specified: bool,
    
    #[serde(rename = "OnOrder")]
    pub on_order: i32,
    
    #[serde(rename = "Level")]
    pub level: i32,
    
    #[serde(rename = "AvailableStock")]
    pub available_stock: i32,
    
    #[serde(rename = "PricePerUnit")]
    pub price_per_unit: f64,
    
    #[serde(rename = "UnitCost")]
    pub unit_cost: f64,
    
    #[serde(rename = "DespatchStockUnitCost")]
    pub despatch_stock_unit_cost: f64,
    
    #[serde(rename = "Discount")]
    pub discount: f64,
    
    #[serde(rename = "Tax")]
    pub tax: f64,
    
    #[serde(rename = "TaxRate")]
    pub tax_rate: f64,
    
    #[serde(rename = "Cost")]
    pub cost: f64,
    
    #[serde(rename = "CostIncTax")]
    pub cost_inc_tax: f64,
    
    #[serde(rename = "CompositeSubItems")]
    pub composite_sub_items: Vec<serde_json::Value>,
    
    #[serde(rename = "IsService")]
    pub is_service: bool,
    
    #[serde(rename = "SalesTax")]
    pub sales_tax: f64,
    
    #[serde(rename = "TaxCostInclusive")]
    pub tax_cost_inclusive: bool,
    
    #[serde(rename = "PartShipped")]
    pub part_shipped: bool,
    
    #[serde(rename = "Weight")]
    pub weight: f64,
    
    #[serde(rename = "BarcodeNumber")]
    pub barcode_number: String,
    
    #[serde(rename = "Market")]
    pub market: i32,
    
    #[serde(rename = "ChannelSKU")]
    pub channel_sku: String,
    
    #[serde(rename = "ChannelTitle")]
    pub channel_title: String,
    
    #[serde(rename = "DiscountValue")]
    pub discount_value: f64,
    
    #[serde(rename = "HasImage")]
    pub has_image: bool,
    
    #[serde(rename = "ImageId")]
    pub image_id: Uuid,
    
    #[serde(rename = "AdditionalInfo")]
    pub additional_info: Vec<serde_json::Value>,
    
    #[serde(rename = "StockLevelIndicator")]
    pub stock_level_indicator: i32,
    
    #[serde(rename = "ShippingCost")]
    pub shipping_cost: f64,
    
    #[serde(rename = "PartShippedQty")]
    pub part_shipped_qty: i32,
    
    #[serde(rename = "BatchNumberScanRequired")]
    pub batch_number_scan_required: bool,
    
    #[serde(rename = "SerialNumberScanRequired")]
    pub serial_number_scan_required: bool,
    
    #[serde(rename = "BinRack")]
    pub bin_rack: String,
    
    #[serde(rename = "BinRacks")]
    pub bin_racks: Vec<BinRack>,
    
    #[serde(rename = "InventoryTrackingType")]
    pub inventory_tracking_type: i32,
    
    #[serde(rename = "isBatchedStockItem")]
    pub is_batched_stock_item: bool,
    
    #[serde(rename = "IsWarehouseManaged")]
    pub is_warehouse_managed: bool,
    
    #[serde(rename = "IsUnlinked")]
    pub is_unlinked: bool,
    
    #[serde(rename = "StockItemIntId")]
    pub stock_item_int_id: i32,
    
    #[serde(rename = "AddedDate")]
    pub added_date: DateTime<Utc>,
    
    #[serde(rename = "RowId")]
    pub row_id: Uuid,
    
    #[serde(rename = "OrderId")]
    pub order_id: Uuid,
    
    #[serde(rename = "StockItemId")]
    pub stock_item_id: Uuid,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Note {
    #[serde(rename = "OrderNoteId")]
    pub order_note_id: Uuid,
    
    #[serde(rename = "OrderId")]
    pub order_id: Uuid,
    
    #[serde(rename = "NoteDate")]
    pub note_date: DateTime<Utc>,
    
    #[serde(rename = "Internal")]
    pub internal: bool,
    
    #[serde(rename = "Note")]
    pub note: String,
    
    #[serde(rename = "CreatedBy")]
    pub created_by: String,
}