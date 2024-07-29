use std::fmt::Display;

use chrono::NaiveDateTime;
use rust_decimal::Decimal;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Bill {
    #[serde(rename = "@Version")]
    pub version: String,

    #[serde(rename = "@Fecha")]
    pub date: NaiveDateTime,

    #[serde(rename = "@FormaPago")]
    pub pay_form: String,

    #[serde(rename = "@SubTotal")]
    pub subtotal: Decimal,

    #[serde(rename = "@Moneda")]
    pub currency: String,

    #[serde(rename = "@Total")]
    pub total: Decimal,

    #[serde(rename = "@TipoDeComprobante")]
    pub receipt_type: String,

    #[serde(rename = "@Exportacion")]
    pub exportation: String,

    #[serde(rename = "@MetodoPago")]
    pub pay_method: String,

    #[serde(rename = "@LugarExpedicion")]
    pub expedition_place: String,

    #[serde(rename = "Emisor")]
    pub emisor: Emisor,

    #[serde(rename = "Receptor")]
    pub recipient: Recipient,

    #[serde(rename = "Conceptos")]
    pub concepts: Concepts,

    #[serde(rename = "Impuestos")]
    pub taxes: Taxes,
}

impl Bill {
    #[inline]
    pub fn total(&self) -> Decimal {
        self.total
    }

    #[inline]
    pub fn subtotal(&self) -> Decimal {
        self.subtotal
    }

    pub fn iva(&self) -> Decimal {
        let mut total = Decimal::ZERO;
        if let Some(w) = &self.taxes.withheld {
            total += w.iva()
        }

        if let Some(c) = &self.taxes.carried_forward {
            total += c.iva()
        }

        total
    }

    pub fn isr(&self) -> Decimal {
        let mut total = Decimal::ZERO;
        if let Some(w) = &self.taxes.withheld {
            total += w.isr()
        }

        if let Some(c) = &self.taxes.carried_forward {
            total += c.isr()
        }

        total
    }
}

#[derive(Debug, Deserialize)]
pub struct Emisor {
    #[serde(rename = "@Rfc")]
    pub rfc: String,
    #[serde(rename = "@Nombre")]
    pub name: String,
    #[serde(rename = "@RegimenFiscal")]
    pub fiscal_regiment: String,
}

#[derive(Debug, Deserialize)]
pub struct Recipient {
    #[serde(rename = "@Rfc")]
    pub rfc: String,

    #[serde(rename = "@Nombre")]
    pub name: String,

    #[serde(rename = "@DomicilioFiscalReceptor")]
    pub zip_code: String,

    #[serde(rename = "@RegimenFiscalReceptor")]
    pub fiscal_regiment: String,

    #[serde(rename = "@UsoCFDI")]
    pub cfdi_use: String,
}

#[derive(Debug, Deserialize)]
pub struct Concepts {
    #[serde(rename = "Concepto")]
    pub list: Vec<Concept>,
}

#[derive(Debug, Deserialize)]
pub struct Concept {
    #[serde(rename = "@ClaveProdServ")]
    pub key: String,

    #[serde(rename = "@Cantidad")]
    pub quantity: Decimal,

    #[serde(rename = "@ClaveUnidad")]
    pub unity_key: String,

    #[serde(rename = "@Unidad")]
    pub unity: String,

    #[serde(rename = "@Descripcion")]
    pub description: String,

    #[serde(rename = "@ValorUnitario")]
    pub unitary_price: Decimal,

    #[serde(rename = "@Importe")]
    pub value: Decimal,
}

#[derive(Debug, Deserialize)]
pub struct Taxes {
    #[serde(rename = "Retenciones")]
    pub withheld: Option<Withheld>,

    #[serde(rename = "Traslados")]
    pub carried_forward: Option<CarriedForward>,
}

#[derive(Debug, Deserialize)]
pub struct Withheld {
    #[serde(rename = "Retencion")]
    pub taxes: Vec<Tax>,
}

impl Withheld {
    fn tax_sum(&self, ty: TaxType) -> Decimal {
        Tax::sum_iter(self.taxes.iter(), ty)
    }

    pub fn iva(&self) -> Decimal {
        self.tax_sum(TaxType::Iva)
    }

    pub fn isr(&self) -> Decimal {
        self.tax_sum(TaxType::Isr)
    }
}

#[derive(Debug, Deserialize)]
pub struct CarriedForward {
    #[serde(rename = "Traslado")]
    pub taxes: Vec<Tax>,
}

impl CarriedForward {
    fn tax_sum(&self, ty: TaxType) -> Decimal {
        Tax::sum_iter(self.taxes.iter(), ty)
    }

    pub fn iva(&self) -> Decimal {
        self.tax_sum(TaxType::Iva)
    }

    pub fn isr(&self) -> Decimal {
        self.tax_sum(TaxType::Isr)
    }
}

#[derive(Debug, Deserialize, Clone, Copy)]
pub struct Tax {
    #[serde(rename = "@Impuesto")]
    pub ty: TaxType,

    #[serde(rename = "@Importe")]
    pub value: Decimal,
}

impl Tax {
    #[inline]
    pub fn sum_iter<'a, I: Iterator<Item = &'a Tax>>(it: I, ty: TaxType) -> Decimal {
        it.fold(Decimal::ZERO, |acc, tax| {
            if tax.ty == ty {
                acc + tax.value
            } else {
                acc
            }
        })
    }
}

#[derive(Debug, Deserialize, PartialEq, Eq, Clone, Copy)]
#[serde(try_from = "String")]
#[serde(into = "String")]
pub enum TaxType {
    /// 001
    Isr,
    /// 002
    Iva,
}

impl Display for TaxType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TaxType::Iva => write!(f, "002"),
            TaxType::Isr => write!(f, "001"),
        }
    }
}

impl TryFrom<String> for TaxType {
    type Error = &'static str;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        match s.as_str() {
            "001" => Ok(Self::Isr),
            "002" => Ok(Self::Iva),
            _ => Err("Impuesto no soportado"),
        }
    }
}
