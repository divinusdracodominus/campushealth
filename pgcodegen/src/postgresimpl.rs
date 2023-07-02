use crate::Error;
use postgres::types::{ToSql};
use postgres::SimpleQueryMessage;
pub trait SimpleClient {
    type Err: std::error::Error;
    type Row;
    fn fetch<S: ToString>(
        &mut self,
        query: S,
        fields: &[&(dyn ToSql + Sync)],
    ) -> Result<Vec<Self::Row>, Self::Err>;
    //fn simple_fetch<S: ToString>(&self, query: S) -> Result<Vec<Option<String>>, Self::Err>;*/
    fn exec<S: ToString>(
        &mut self,
        query: S,
        fields: &[&(dyn ToSql + Sync)],
    ) -> Result<u64, Self::Err>;
    //fn simple_exec<S: ToString>(&self, query: S) -> Result<u64, Self::Err>;
}

impl SimpleClient for postgres::Client {
    type Err = postgres::Error;
    type Row = postgres::Row;
    fn exec<S: ToString>(&mut self, query: S,
    fields: &[&(dyn ToSql + Sync)]) -> Result<u64, Self::Err> {
        self.execute(&query.to_string(), fields)
    }

    fn fetch<'a, S: ToString>(
        &mut self,
        query: S,
        fields: &[&(dyn ToSql + Sync)],
    ) -> Result<Vec<Self::Row>, Self::Err> {
        self.query(&query.to_string(), fields)
    }
}

pub fn from_query_message<'a>(
    message: SimpleQueryMessage,
) -> Result<Vec<Option<String>>, crate::Error> {
    Ok(match message {
        SimpleQueryMessage::Row(row) => {
            let mut outvec = Vec::with_capacity(row.len());
            for i in 0..row.len() {
                outvec.push(row.try_get(i)?.map(|v| v.to_string()));
            }
            outvec
        }
        SimpleQueryMessage::CommandComplete(v) => return Err(crate::Error::CommandComplete(v)),
        _ => panic!("unhandled result"),
    })
}

pub fn get_column<'a>(
    message: Result<Vec<Option<String>>, crate::Error>,
    idx: usize,
) -> Result<Option<String>, crate::Error> {
    match message {
        Ok(val) => match val
            .get(idx)
        {
            Some(val) => match val.as_ref() {
                Some(val) => Ok(Some(val.to_string())),
                None => Ok(None),
            },
            None => Ok(None),
        },
        Err(e) => match e {
            crate::Error::CommandComplete(_) => return Ok(None),
            _ => return Err(e),
        },
    }
}