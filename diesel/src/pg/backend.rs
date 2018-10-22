//! The PostgreSQL backend

use byteorder::NetworkEndian;

use super::query_builder::PgQueryBuilder;
use super::PgMetadataLookup;
use backend::*;
use prelude::Queryable;
use query_builder::bind_collector::RawBytesBindCollector;
use sql_types::{Oid, TypeMetadata};

/// The PostgreSQL backend
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct Pg;

/// The raw value representation of the postgres backend
#[derive(Debug, Clone)]
pub struct PgValue<'a> {
    data: &'a [u8],
    oid: u32,
}

impl<'a> PgValue<'a> {
    pub(crate) fn new(bytes: &'a [u8], oid: u32) -> PgValue<'a> {
        PgValue { data: bytes, oid }
    }

    /// Get the bytes associated with this raw value
    pub fn bytes(&self) -> &[u8] {
        &self.data
    }

    /// Get the type oid of the value represented by this raw value
    pub fn oid(&self) -> u32 {
        self.oid
    }
}

#[doc(hidden)]
#[derive(Debug, Clone, Copy)]
pub struct PgValueHelper;

impl<'a> RefHelper<'a> for PgValueHelper {
    type Out = PgValue<'a>;
}

/// The [OIDs] for a SQL type
///
/// [OIDs]: https://www.postgresql.org/docs/current/static/datatype-oid.html
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Default)]
pub struct PgTypeMetadata {
    /// The [OID] of `T`
    ///
    /// [OID]: https://www.postgresql.org/docs/current/static/datatype-oid.html
    pub oid: u32,
    /// The [OID] of `T[]`
    ///
    /// [OID]: https://www.postgresql.org/docs/current/static/datatype-oid.html
    pub array_oid: u32,
}

impl Queryable<(Oid, Oid), Pg> for PgTypeMetadata {
    type Row = (u32, u32);

    fn build((oid, array_oid): Self::Row) -> Self {
        PgTypeMetadata { oid, array_oid }
    }
}

impl Backend for Pg {
    type QueryBuilder = PgQueryBuilder;
    type BindCollector = RawBytesBindCollector<Pg>;
    type RawValue = PgValueHelper;
    type ByteOrder = NetworkEndian;
}

impl TypeMetadata for Pg {
    type TypeMetadata = PgTypeMetadata;
    type MetadataLookup = PgMetadataLookup;
}

impl SupportsReturningClause for Pg {}
impl SupportsDefaultKeyword for Pg {}
impl UsesAnsiSavepointSyntax for Pg {}
