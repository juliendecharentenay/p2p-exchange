
macro_rules! count {
  ( $class:ident, $class_sql:ident, $data:ident ) => {
    /// Count all
    pub async fn count(data: &$data) -> Result<usize, Box<dyn std::error::Error>> {
      let db = data.db()?;
      let sql = $class_sql::from_rusqlite(&db)?;
      sql.create_table()?;
      Ok(sql.count_all()?)
    }
  };
}
pub(crate) use count;

macro_rules! list {
  ( $class:ident, $class_sql:ident, $data:ident ) => {
    /// List all
    pub async fn list(data: &$data) -> Result<Vec<$class>, Box<dyn std::error::Error>> {
      let db = data.db()?;
      let sql = $class_sql::from_rusqlite(&db)?;
      sql.create_table()?;
      let filter = Filter::TimestampGreaterEqualThan(chrono::Utc::now() - chrono::Duration::minutes(5));
      Ok(sql.select(filter.into())?)
    }
  };

  ( $class:ident, $class_sql:ident, $data:ident, $info:ident ) => {
    /// List with a query string
    pub async fn list(data: &$data, info: $info) -> Result<Vec<$class>, Box<dyn std::error::Error>> {
      let db = data.db()?;
      let sql = $class_sql::from_rusqlite(&db)?;
      sql.create_table()?;
      let filter = Filter::TimestampGreaterEqualThan(chrono::Utc::now() - chrono::Duration::minutes(5));
      let filter = match info.into() {
          Some(i) => Filter::And(Box::new(i), Box::new(filter)),
          None => filter,
      };
      Ok(sql.select(filter.into())?)
    }
  };

}
pub(crate) use list;

macro_rules! post {
  ( $class:ident, $class_sql:ident, $data:ident ) => {
    pub async fn post(data: &$data, body: &String) -> Result<$class, Box<dyn std::error::Error>> {
      let mut item: $class = serde_json::from_str(body)?;
      let db = data.db()?;
      let sql = $class_sql::from_rusqlite(&db)?;
      sql.create_table()?;
      item.id = crate::gen_key();
      item.timestamp = chrono::Utc::now();
      sql.insert(&item)?;
      Ok(item)
    }
  };
}
pub(crate) use post;

macro_rules! get {
  ( $class:ident, $class_sql:ident, $data:ident ) => {
    pub async fn get(data: &$data, id: &String) -> Result<Option<$class>, Box<dyn std::error::Error>> {
      let db = data.db()?;
      let sql = $class_sql::from_rusqlite(&db)?;
      sql.create_table()?;
      let filter = Filter::TimestampGreaterEqualThan(chrono::Utc::now() - chrono::Duration::minutes(5));
      Ok(sql.select_one(Filter::And(Box::new(Filter::IdEqual(id.clone())), Box::new(filter)).into())?)
    }
  };
}
pub(crate) use get;

macro_rules! update {
  ( $class:ident, $class_sql:ident, $data:ident ) => {
    pub async fn update(data: &$data, id: &String, body: &String) -> Result<Option<$class>, Box<dyn std::error::Error>> {
      let db = data.db()?;
      let sql = $class_sql::from_rusqlite(&db)?;
      sql.create_table()?;
      let filter = Filter::TimestampGreaterEqualThan(chrono::Utc::now() - chrono::Duration::minutes(5));
      let previous = sql.select_one(Filter::And(Box::new(Filter::IdEqual(id.clone())), Box::new(filter)).into())?;
      if previous.is_some() {
        let previous = previous.as_ref().unwrap();
        let mut new: $class = serde_json::from_str(body)?;
        new.id = previous.id.clone();
        sql.update_to(previous, &new)?;
      }
      Ok(previous)
    }
  };
}
pub(crate) use update;

macro_rules! delete {
  ( $class:ident, $class_sql:ident, $data:ident ) => {
    pub async fn delete(data: &$data, id: &String) -> Result<Option<$class>, Box<dyn std::error::Error>> {
      let db = data.db()?;
      let sql = $class_sql::from_rusqlite(&db)?;
      sql.create_table()?;
      let item = sql.select_one(Filter::IdEqual(id.clone()).into())?;
      sql.delete(Filter::IdEqual(id.clone()).into())?;
      Ok(item)
    }
  };
}
pub(crate) use delete;


