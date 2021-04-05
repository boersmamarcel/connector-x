use chrono::{DateTime, NaiveDate, NaiveDateTime, Utc};
use postgres::types::Type;
use rust_decimal::Decimal;

#[derive(Copy, Clone, Debug)]
pub enum PostgresTypeSystem {
    Bool(bool),
    Float4(bool),
    Float8(bool),
    Numeric(bool),
    Int4(bool),
    Int8(bool),
    Date(bool),
    BpChar(bool),
    VarChar(bool),
    Text(bool),
    Timestamp(bool),
    TimestampTz(bool),
}

impl_typesystem! {
    system = PostgresTypeSystem,
    mappings = {
        { Int4 => i32 }
        { Int8 => i64 }
        { Float4 => f32 }
        { Float8 => f64 }
        { Numeric => Decimal }
        { Bool => bool }
        { Text | BpChar | VarChar => &'r str }
        { Timestamp => NaiveDateTime }
        { TimestampTz => DateTime<Utc> }
        { Date => NaiveDate }
    }
}

impl<'a> From<&'a Type> for PostgresTypeSystem {
    fn from(ty: &'a Type) -> PostgresTypeSystem {
        use PostgresTypeSystem::*;
        match ty.name() {
            "int4" => Int4(true),
            "int8" => Int8(true),
            "float4" => Float4(true),
            "float8" => Float8(true),
            "numeric" => Numeric(true),
            "bool" => Bool(true),
            "text" => Text(true),
            "bpchar" => BpChar(true),
            "varchar" => VarChar(true),
            "timestamp" => TimestampTz(true),
            "timestamptz" => Timestamp(true),
            "date" => Date(true),
            ty => unimplemented!("{}", ty),
        }
    }
}

// Link PostgresDTypes back to the one defiend by the postgres crate.
impl<'a> From<PostgresTypeSystem> for Type {
    fn from(ty: PostgresTypeSystem) -> Type {
        use PostgresTypeSystem::*;
        match ty {
            Int4(_) => Type::INT4,
            Int8(_) => Type::INT8,
            Float4(_) => Type::FLOAT4,
            Float8(_) => Type::FLOAT8,
            Numeric(_) => Type::NUMERIC,
            Bool(_) => Type::BOOL,
            Text(_) => Type::TEXT,
            BpChar(_) => Type::BPCHAR,
            VarChar(_) => Type::VARCHAR,
            Timestamp(_) => Type::TIMESTAMP,
            TimestampTz(_) => Type::TIMESTAMPTZ,
            Date(_) => Type::DATE,
        }
    }
}