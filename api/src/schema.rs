
            async fn create_client() -> tokio_postgres::Client {
                let (mut client, conn) = tokio_postgres::connect("postgresql://cardinal:Qksg0FV2EMDM@192.168.122.1:5432/myhealth", tokio_postgres::NoTls).await.unwrap();

                tokio::spawn(async move {
                    if let Err(error) = conn.await {
                      eprintln!("Connection error: {}", error);
                    }
                  });

                client
            }
        

/// base64 encoded hash of table names, column names, and column types using alphanumerics, ~, _
pub fn version_hash() -> &'static str {
    "n19_f0FTKzs="
}

#[async_trait::async_trait]
/// the primary trait that is used for ORM CRUD operations, ## NOTE: Future plans include a default impl for insert_one, update_one, delete_one, select_one that use self.into::<WhereSelf>()
pub trait DBRow<T>
where Self: Sized,
{
    type Err: std::error::Error;

    async fn delete(&self, client: &mut T) -> Result<u64, Self::Err>;

    async fn insert(&self, client: &mut T) -> Result<u64, Self::Err>;

    /// creates Self based on the primary key value passed into the function
    async fn select<P>(client: &mut T, pkey: &P) -> Result<Option<Self>, Self::Err>
    where P: tokio_postgres::types::ToSql,
          P: std::marker::Sync,
    ;

    async fn update(&self, client: &mut T) -> Result<u64, Self::Err>;
}

#[async_trait::async_trait]
impl DBRow<tokio_postgres::Client> for eventrel {
    type Err = tokio_postgres::Error;

    async fn delete(&self, client: &mut tokio_postgres::Client) -> Result<u64, Self::Err> {
        client.execute("delete from eventrel where id = $1", &[&self.id]).await
    }

    async fn insert(&self, client: &mut tokio_postgres::Client) -> Result<u64, Self::Err> {
        client.execute("insert into eventrel (id, source, key) values ($1, $2, $3)", &[&self.id, &self.source, &self.key]).await
    }

    /// creates Self based on the primary key value passed into the function
    async fn select<P>(client: &mut tokio_postgres::Client, pkey: &P) -> Result<Option<Self>, Self::Err>
    where P: tokio_postgres::types::ToSql,
          P: std::marker::Sync,
    {
        let selected = client.query("select id, source, key from eventrel where id = $1", &[pkey]).await?;
        let fields = match selected.get(0) {
        Some(fields) => fields,
        None => return Ok(None)
        };
        Ok(Some(Self { 
        id: fields.try_get(0)?,
        source: fields.try_get(1)?,
        key: fields.try_get(2)?,
        }))
    }

    async fn update(&self, client: &mut tokio_postgres::Client) -> Result<u64, Self::Err> {
        client.execute("update eventrel set id = $1, source = $2, key = $3 where id = $4", &[&self.id, &self.source, &self.key, &self.id]).await
    }
}

#[async_trait::async_trait]
impl DBRow<tokio_postgres::Client> for attendance {
    type Err = tokio_postgres::Error;

    async fn delete(&self, client: &mut tokio_postgres::Client) -> Result<u64, Self::Err> {
        client.execute("delete from attendance where id = $1", &[&self.id]).await
    }

    async fn insert(&self, client: &mut tokio_postgres::Client) -> Result<u64, Self::Err> {
        client.execute("insert into attendance (participantid, instanceid, id, status) values ($1, $2, $3, $4)", &[&self.participantid, &self.instanceid, &self.id, &self.status]).await
    }

    /// creates Self based on the primary key value passed into the function
    async fn select<P>(client: &mut tokio_postgres::Client, pkey: &P) -> Result<Option<Self>, Self::Err>
    where P: tokio_postgres::types::ToSql,
          P: std::marker::Sync,
    {
        let selected = client.query("select participantid, instanceid, id, status from attendance where id = $1", &[pkey]).await?;
        let fields = match selected.get(0) {
        Some(fields) => fields,
        None => return Ok(None)
        };
        Ok(Some(Self { 
        participantid: fields.try_get(0)?,
        instanceid: fields.try_get(1)?,
        id: fields.try_get(2)?,
        status: fields.try_get(3)?,
        }))
    }

    async fn update(&self, client: &mut tokio_postgres::Client) -> Result<u64, Self::Err> {
        client.execute("update attendance set participantid = $1, instanceid = $2, id = $3, status = $4 where id = $5", &[&self.participantid, &self.instanceid, &self.id, &self.status, &self.id]).await
    }
}

#[async_trait::async_trait]
impl DBRow<tokio_postgres::Client> for participants {
    type Err = tokio_postgres::Error;

    async fn delete(&self, client: &mut tokio_postgres::Client) -> Result<u64, Self::Err> {
        client.execute("delete from participants where id = $1", &[&self.id]).await
    }

    async fn insert(&self, client: &mut tokio_postgres::Client) -> Result<u64, Self::Err> {
        client.execute("insert into participants (id, participant_id, first_name, last_name, email) values ($1, $2, $3, $4, $5)", &[&self.id, &self.participant_id, &self.first_name, &self.last_name, &self.email]).await
    }

    /// creates Self based on the primary key value passed into the function
    async fn select<P>(client: &mut tokio_postgres::Client, pkey: &P) -> Result<Option<Self>, Self::Err>
    where P: tokio_postgres::types::ToSql,
          P: std::marker::Sync,
    {
        let selected = client.query("select id, participant_id, first_name, last_name, email from participants where id = $1", &[pkey]).await?;
        let fields = match selected.get(0) {
        Some(fields) => fields,
        None => return Ok(None)
        };
        Ok(Some(Self { 
        id: fields.try_get(0)?,
        participant_id: fields.try_get(1)?,
        first_name: fields.try_get(2)?,
        last_name: fields.try_get(3)?,
        email: fields.try_get(4)?,
        }))
    }

    async fn update(&self, client: &mut tokio_postgres::Client) -> Result<u64, Self::Err> {
        client.execute("update participants set id = $1, participant_id = $2, first_name = $3, last_name = $4, email = $5 where id = $6", &[&self.id, &self.participant_id, &self.first_name, &self.last_name, &self.email, &self.id]).await
    }
}

#[async_trait::async_trait]
impl DBRow<tokio_postgres::Client> for calendarinstance {
    type Err = tokio_postgres::Error;

    async fn delete(&self, client: &mut tokio_postgres::Client) -> Result<u64, Self::Err> {
        client.execute("delete from calendarinstance where id = $1", &[&self.id]).await
    }

    async fn insert(&self, client: &mut tokio_postgres::Client) -> Result<u64, Self::Err> {
        client.execute("insert into calendarinstance (id, calendar_id, begin, end) values ($1, $2, $3, $4)", &[&self.id, &self.calendar_id, &self.begin, &self.end]).await
    }

    /// creates Self based on the primary key value passed into the function
    async fn select<P>(client: &mut tokio_postgres::Client, pkey: &P) -> Result<Option<Self>, Self::Err>
    where P: tokio_postgres::types::ToSql,
          P: std::marker::Sync,
    {
        let selected = client.query("select id, calendar_id, begin, end from calendarinstance where id = $1", &[pkey]).await?;
        let fields = match selected.get(0) {
        Some(fields) => fields,
        None => return Ok(None)
        };
        Ok(Some(Self { 
        id: fields.try_get(0)?,
        calendar_id: fields.try_get(1)?,
        begin: fields.try_get(2)?,
        end: fields.try_get(3)?,
        }))
    }

    async fn update(&self, client: &mut tokio_postgres::Client) -> Result<u64, Self::Err> {
        client.execute("update calendarinstance set id = $1, calendar_id = $2, begin = $3, end = $4 where id = $5", &[&self.id, &self.calendar_id, &self.begin, &self.end, &self.id]).await
    }
}

#[async_trait::async_trait]
impl DBRow<tokio_postgres::Client> for appusageevents {
    type Err = tokio_postgres::Error;

    async fn delete(&self, client: &mut tokio_postgres::Client) -> Result<u64, Self::Err> {
        client.execute("delete from appusageevents where id = $1", &[&self.id]).await
    }

    async fn insert(&self, client: &mut tokio_postgres::Client) -> Result<u64, Self::Err> {
        client.execute("insert into appusageevents (metadata_id, platform_type, date, id, kind, package_name, minute) values ($1, $2, $3, $4, $5, $6, $7)", &[&self.metadata_id, &self.platform_type, &self.date, &self.id, &self.kind, &self.package_name, &self.minute]).await
    }

    /// creates Self based on the primary key value passed into the function
    async fn select<P>(client: &mut tokio_postgres::Client, pkey: &P) -> Result<Option<Self>, Self::Err>
    where P: tokio_postgres::types::ToSql,
          P: std::marker::Sync,
    {
        let selected = client.query("select metadata_id, platform_type, date, id, kind, package_name, minute from appusageevents where id = $1", &[pkey]).await?;
        let fields = match selected.get(0) {
        Some(fields) => fields,
        None => return Ok(None)
        };
        Ok(Some(Self { 
        metadata_id: fields.try_get(0)?,
        platform_type: fields.try_get(1)?,
        date: fields.try_get(2)?,
        id: fields.try_get(3)?,
        kind: fields.try_get(4)?,
        package_name: fields.try_get(5)?,
        minute: fields.try_get(6)?,
        }))
    }

    async fn update(&self, client: &mut tokio_postgres::Client) -> Result<u64, Self::Err> {
        client.execute("update appusageevents set metadata_id = $1, platform_type = $2, date = $3, id = $4, kind = $5, package_name = $6, minute = $7 where id = $8", &[&self.metadata_id, &self.platform_type, &self.date, &self.id, &self.kind, &self.package_name, &self.minute, &self.id]).await
    }
}

#[async_trait::async_trait]
impl DBRow<tokio_postgres::Client> for events {
    type Err = tokio_postgres::Error;

    async fn delete(&self, client: &mut tokio_postgres::Client) -> Result<u64, Self::Err> {
        client.execute("delete from events where id = $1", &[&self.id]).await
    }

    async fn insert(&self, client: &mut tokio_postgres::Client) -> Result<u64, Self::Err> {
        client.execute("insert into events (repeats, id, organizer, description, repitition, days_of_week, image_url, color, image_description, title) values ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)", &[&self.repeats, &self.id, &self.organizer, &self.description, &self.repitition, &self.days_of_week, &self.image_url, &self.color, &self.image_description, &self.title]).await
    }

    /// creates Self based on the primary key value passed into the function
    async fn select<P>(client: &mut tokio_postgres::Client, pkey: &P) -> Result<Option<Self>, Self::Err>
    where P: tokio_postgres::types::ToSql,
          P: std::marker::Sync,
    {
        let selected = client.query("select repeats, id, organizer, description, repitition, days_of_week, image_url, color, image_description, title from events where id = $1", &[pkey]).await?;
        let fields = match selected.get(0) {
        Some(fields) => fields,
        None => return Ok(None)
        };
        Ok(Some(Self { 
        repeats: fields.try_get(0)?,
        id: fields.try_get(1)?,
        organizer: fields.try_get(2)?,
        description: fields.try_get(3)?,
        repitition: fields.try_get(4)?,
        days_of_week: fields.try_get(5)?,
        image_url: fields.try_get(6)?,
        color: fields.try_get(7)?,
        image_description: fields.try_get(8)?,
        title: fields.try_get(9)?,
        }))
    }

    async fn update(&self, client: &mut tokio_postgres::Client) -> Result<u64, Self::Err> {
        client.execute("update events set repeats = $1, id = $2, organizer = $3, description = $4, repitition = $5, days_of_week = $6, image_url = $7, color = $8, image_description = $9, title = $10 where id = $11", &[&self.repeats, &self.id, &self.organizer, &self.description, &self.repitition, &self.days_of_week, &self.image_url, &self.color, &self.image_description, &self.title, &self.id]).await
    }
}

#[async_trait::async_trait]
impl DBRow<tokio_postgres::Client> for smsdata {
    type Err = tokio_postgres::Error;

    async fn delete(&self, client: &mut tokio_postgres::Client) -> Result<u64, Self::Err> {
        client.execute("delete from smsdata where id = $1", &[&self.id]).await
    }

    async fn insert(&self, client: &mut tokio_postgres::Client) -> Result<u64, Self::Err> {
        client.execute("insert into smsdata (inbound, platform_id, date_sent, id, date, thread_id, metadata_id, minute, body, address, recipient_id) values ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)", &[&self.inbound, &self.platform_id, &self.date_sent, &self.id, &self.date, &self.thread_id, &self.metadata_id, &self.minute, &self.body, &self.address, &self.recipient_id]).await
    }

    /// creates Self based on the primary key value passed into the function
    async fn select<P>(client: &mut tokio_postgres::Client, pkey: &P) -> Result<Option<Self>, Self::Err>
    where P: tokio_postgres::types::ToSql,
          P: std::marker::Sync,
    {
        let selected = client.query("select inbound, platform_id, date_sent, id, date, thread_id, metadata_id, minute, body, address, recipient_id from smsdata where id = $1", &[pkey]).await?;
        let fields = match selected.get(0) {
        Some(fields) => fields,
        None => return Ok(None)
        };
        Ok(Some(Self { 
        inbound: fields.try_get(0)?,
        platform_id: fields.try_get(1)?,
        date_sent: fields.try_get(2)?,
        id: fields.try_get(3)?,
        date: fields.try_get(4)?,
        thread_id: fields.try_get(5)?,
        metadata_id: fields.try_get(6)?,
        minute: fields.try_get(7)?,
        body: fields.try_get(8)?,
        address: fields.try_get(9)?,
        recipient_id: fields.try_get(10)?,
        }))
    }

    async fn update(&self, client: &mut tokio_postgres::Client) -> Result<u64, Self::Err> {
        client.execute("update smsdata set inbound = $1, platform_id = $2, date_sent = $3, id = $4, date = $5, thread_id = $6, metadata_id = $7, minute = $8, body = $9, address = $10, recipient_id = $11 where id = $12", &[&self.inbound, &self.platform_id, &self.date_sent, &self.id, &self.date, &self.thread_id, &self.metadata_id, &self.minute, &self.body, &self.address, &self.recipient_id, &self.id]).await
    }
}

#[async_trait::async_trait]
impl DBRow<tokio_postgres::Client> for calldata {
    type Err = tokio_postgres::Error;

    async fn delete(&self, client: &mut tokio_postgres::Client) -> Result<u64, Self::Err> {
        client.execute("delete from calldata where id = $1", &[&self.id]).await
    }

    async fn insert(&self, client: &mut tokio_postgres::Client) -> Result<u64, Self::Err> {
        client.execute("insert into calldata (id, metadata_id, date, duration, recipiend_id) values ($1, $2, $3, $4, $5)", &[&self.id, &self.metadata_id, &self.date, &self.duration, &self.recipiend_id]).await
    }

    /// creates Self based on the primary key value passed into the function
    async fn select<P>(client: &mut tokio_postgres::Client, pkey: &P) -> Result<Option<Self>, Self::Err>
    where P: tokio_postgres::types::ToSql,
          P: std::marker::Sync,
    {
        let selected = client.query("select id, metadata_id, date, duration, recipiend_id from calldata where id = $1", &[pkey]).await?;
        let fields = match selected.get(0) {
        Some(fields) => fields,
        None => return Ok(None)
        };
        Ok(Some(Self { 
        id: fields.try_get(0)?,
        metadata_id: fields.try_get(1)?,
        date: fields.try_get(2)?,
        duration: fields.try_get(3)?,
        recipiend_id: fields.try_get(4)?,
        }))
    }

    async fn update(&self, client: &mut tokio_postgres::Client) -> Result<u64, Self::Err> {
        client.execute("update calldata set id = $1, metadata_id = $2, date = $3, duration = $4, recipiend_id = $5 where id = $6", &[&self.id, &self.metadata_id, &self.date, &self.duration, &self.recipiend_id, &self.id]).await
    }
}

#[async_trait::async_trait]
impl DBRow<tokio_postgres::Client> for metadata {
    type Err = tokio_postgres::Error;

    async fn delete(&self, client: &mut tokio_postgres::Client) -> Result<u64, Self::Err> {
        client.execute("delete from metadata where id = $1", &[&self.id]).await
    }

    async fn insert(&self, client: &mut tokio_postgres::Client) -> Result<u64, Self::Err> {
        client.execute("insert into metadata (id, participant_id, device_id) values ($1, $2, $3)", &[&self.id, &self.participant_id, &self.device_id]).await
    }

    /// creates Self based on the primary key value passed into the function
    async fn select<P>(client: &mut tokio_postgres::Client, pkey: &P) -> Result<Option<Self>, Self::Err>
    where P: tokio_postgres::types::ToSql,
          P: std::marker::Sync,
    {
        let selected = client.query("select id, participant_id, device_id from metadata where id = $1", &[pkey]).await?;
        let fields = match selected.get(0) {
        Some(fields) => fields,
        None => return Ok(None)
        };
        Ok(Some(Self { 
        id: fields.try_get(0)?,
        participant_id: fields.try_get(1)?,
        device_id: fields.try_get(2)?,
        }))
    }

    async fn update(&self, client: &mut tokio_postgres::Client) -> Result<u64, Self::Err> {
        client.execute("update metadata set id = $1, participant_id = $2, device_id = $3 where id = $4", &[&self.id, &self.participant_id, &self.device_id, &self.id]).await
    }
}

#[async_trait::async_trait]
impl DBRow<tokio_postgres::Client> for eventinstances {
    type Err = tokio_postgres::Error;

    async fn delete(&self, client: &mut tokio_postgres::Client) -> Result<u64, Self::Err> {
        client.execute("delete from eventinstances where id = $1", &[&self.id]).await
    }

    async fn insert(&self, client: &mut tokio_postgres::Client) -> Result<u64, Self::Err> {
        client.execute("insert into eventinstances (id, event_id, begin, end, duration) values ($1, $2, $3, $4, $5)", &[&self.id, &self.event_id, &self.begin, &self.end, &self.duration]).await
    }

    /// creates Self based on the primary key value passed into the function
    async fn select<P>(client: &mut tokio_postgres::Client, pkey: &P) -> Result<Option<Self>, Self::Err>
    where P: tokio_postgres::types::ToSql,
          P: std::marker::Sync,
    {
        let selected = client.query("select id, event_id, begin, end, duration from eventinstances where id = $1", &[pkey]).await?;
        let fields = match selected.get(0) {
        Some(fields) => fields,
        None => return Ok(None)
        };
        Ok(Some(Self { 
        id: fields.try_get(0)?,
        event_id: fields.try_get(1)?,
        begin: fields.try_get(2)?,
        end: fields.try_get(3)?,
        duration: fields.try_get(4)?,
        }))
    }

    async fn update(&self, client: &mut tokio_postgres::Client) -> Result<u64, Self::Err> {
        client.execute("update eventinstances set id = $1, event_id = $2, begin = $3, end = $4, duration = $5 where id = $6", &[&self.id, &self.event_id, &self.begin, &self.end, &self.duration, &self.id]).await
    }
}

#[async_trait::async_trait]
impl DBRow<tokio_postgres::Client> for calendarentry {
    type Err = tokio_postgres::Error;

    async fn delete(&self, client: &mut tokio_postgres::Client) -> Result<u64, Self::Err> {
        client.execute("delete from calendarentry where id = $1", &[&self.id]).await
    }

    async fn insert(&self, client: &mut tokio_postgres::Client) -> Result<u64, Self::Err> {
        client.execute("insert into calendarentry (metadata, id, selfattendeestatus, eventlocation, owneraccount, title, description) values ($1, $2, $3, $4, $5, $6, $7)", &[&self.metadata, &self.id, &self.selfattendeestatus, &self.eventlocation, &self.owneraccount, &self.title, &self.description]).await
    }

    /// creates Self based on the primary key value passed into the function
    async fn select<P>(client: &mut tokio_postgres::Client, pkey: &P) -> Result<Option<Self>, Self::Err>
    where P: tokio_postgres::types::ToSql,
          P: std::marker::Sync,
    {
        let selected = client.query("select metadata, id, selfattendeestatus, eventlocation, owneraccount, title, description from calendarentry where id = $1", &[pkey]).await?;
        let fields = match selected.get(0) {
        Some(fields) => fields,
        None => return Ok(None)
        };
        Ok(Some(Self { 
        metadata: fields.try_get(0)?,
        id: fields.try_get(1)?,
        selfattendeestatus: fields.try_get(2)?,
        eventlocation: fields.try_get(3)?,
        owneraccount: fields.try_get(4)?,
        title: fields.try_get(5)?,
        description: fields.try_get(6)?,
        }))
    }

    async fn update(&self, client: &mut tokio_postgres::Client) -> Result<u64, Self::Err> {
        client.execute("update calendarentry set metadata = $1, id = $2, selfattendeestatus = $3, eventlocation = $4, owneraccount = $5, title = $6, description = $7 where id = $8", &[&self.metadata, &self.id, &self.selfattendeestatus, &self.eventlocation, &self.owneraccount, &self.title, &self.description, &self.id]).await
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct eventrel {
    pub id: uuid::Uuid,
    pub source: String,
    pub key: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct attendance {
    pub participantid: Option<uuid::Uuid>,
    pub instanceid: Option<uuid::Uuid>,
    pub id: uuid::Uuid,
    pub status: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct participants {
    pub id: uuid::Uuid,
    pub participant_id: i32,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct calendarinstance {
    pub id: i32,
    pub calendar_id: i32,
    pub begin: i64,
    pub end: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct appusageevents {
    pub metadata_id: Option<uuid::Uuid>,
    pub platform_type: Option<i32>,
    pub date: Option<i64>,
    pub id: uuid::Uuid,
    pub kind: Option<String>,
    pub package_name: Option<String>,
    pub minute: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct events {
    pub repeats: Option<bool>,
    pub id: uuid::Uuid,
    pub organizer: Option<String>,
    pub description: Option<String>,
    pub repitition: Option<String>,
    pub days_of_week: Option<Vec<String>>,
    pub image_url: Option<String>,
    pub color: Option<String>,
    pub image_description: Option<String>,
    pub title: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct smsdata {
    pub inbound: bool,
    pub platform_id: Option<i32>,
    pub date_sent: Option<i64>,
    pub id: uuid::Uuid,
    pub date: Option<i64>,
    pub thread_id: i32,
    pub metadata_id: Option<uuid::Uuid>,
    pub minute: Option<String>,
    pub body: Option<String>,
    pub address: Option<String>,
    pub recipient_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct calldata {
    pub id: uuid::Uuid,
    pub metadata_id: Option<uuid::Uuid>,
    pub date: Option<i64>,
    pub duration: Option<i32>,
    pub recipiend_id: Option<uuid::Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct metadata {
    pub id: uuid::Uuid,
    pub participant_id: Option<uuid::Uuid>,
    pub device_id: Option<uuid::Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct eventinstances {
    pub id: uuid::Uuid,
    pub event_id: uuid::Uuid,
    pub begin: i64,
    pub end: i64,
    pub duration: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct calendarentry {
    pub metadata: Option<uuid::Uuid>,
    pub id: i64,
    pub selfattendeestatus: Option<bool>,
    pub eventlocation: Option<String>,
    pub owneraccount: Option<String>,
    pub title: String,
    pub description: Option<String>,
}

pub fn mount_routes(mut rocket: rocket::Rocket<rocket::Build>) -> rocket::Rocket<rocket::Build> {
    rocket
       .mount("/api/", rocket::routes![eventrel_rocket_get])
       .mount("/api/", rocket::routes![attendance_rocket_get])
       .mount("/api/", rocket::routes![participants_rocket_get])
       .mount("/api/", rocket::routes![calendarinstance_rocket_get])
       .mount("/api/", rocket::routes![appusageevents_rocket_get])
       .mount("/api/", rocket::routes![events_rocket_get])
       .mount("/api/", rocket::routes![smsdata_rocket_get])
       .mount("/api/", rocket::routes![calldata_rocket_get])
       .mount("/api/", rocket::routes![metadata_rocket_get])
       .mount("/api/", rocket::routes![eventinstances_rocket_get])
       .mount("/api/", rocket::routes![calendarentry_rocket_get])
}

#[rocket::get("/eventrel/<id>", format = "json")]
async fn eventrel_rocket_get(id: uuid::Uuid) -> rocket::response::content::RawJson<String> {
    let mut client = create_client().await;
    rocket::response::content::RawJson(serde_json::to_string(&eventrel::select(&mut client, &id).await.unwrap().unwrap()).unwrap())
}

#[rocket::get("/attendance/<id>", format = "json")]
async fn attendance_rocket_get(id: uuid::Uuid) -> rocket::response::content::RawJson<String> {
    let mut client = create_client().await;
    rocket::response::content::RawJson(serde_json::to_string(&attendance::select(&mut client, &id).await.unwrap().unwrap()).unwrap())
}

#[rocket::get("/participants/<id>", format = "json")]
async fn participants_rocket_get(id: uuid::Uuid) -> rocket::response::content::RawJson<String> {
    let mut client = create_client().await;
    rocket::response::content::RawJson(serde_json::to_string(&participants::select(&mut client, &id).await.unwrap().unwrap()).unwrap())
}

#[rocket::get("/calendarinstance/<id>", format = "json")]
async fn calendarinstance_rocket_get(id: i32) -> rocket::response::content::RawJson<String> {
    let mut client = create_client().await;
    rocket::response::content::RawJson(serde_json::to_string(&calendarinstance::select(&mut client, &id).await.unwrap().unwrap()).unwrap())
}

#[rocket::get("/appusageevents/<id>", format = "json")]
async fn appusageevents_rocket_get(id: uuid::Uuid) -> rocket::response::content::RawJson<String> {
    let mut client = create_client().await;
    rocket::response::content::RawJson(serde_json::to_string(&appusageevents::select(&mut client, &id).await.unwrap().unwrap()).unwrap())
}

#[rocket::get("/events/<id>", format = "json")]
async fn events_rocket_get(id: uuid::Uuid) -> rocket::response::content::RawJson<String> {
    let mut client = create_client().await;
    rocket::response::content::RawJson(serde_json::to_string(&events::select(&mut client, &id).await.unwrap().unwrap()).unwrap())
}

#[rocket::get("/smsdata/<id>", format = "json")]
async fn smsdata_rocket_get(id: uuid::Uuid) -> rocket::response::content::RawJson<String> {
    let mut client = create_client().await;
    rocket::response::content::RawJson(serde_json::to_string(&smsdata::select(&mut client, &id).await.unwrap().unwrap()).unwrap())
}

#[rocket::get("/calldata/<id>", format = "json")]
async fn calldata_rocket_get(id: uuid::Uuid) -> rocket::response::content::RawJson<String> {
    let mut client = create_client().await;
    rocket::response::content::RawJson(serde_json::to_string(&calldata::select(&mut client, &id).await.unwrap().unwrap()).unwrap())
}

#[rocket::get("/metadata/<id>", format = "json")]
async fn metadata_rocket_get(id: uuid::Uuid) -> rocket::response::content::RawJson<String> {
    let mut client = create_client().await;
    rocket::response::content::RawJson(serde_json::to_string(&metadata::select(&mut client, &id).await.unwrap().unwrap()).unwrap())
}

#[rocket::get("/eventinstances/<id>", format = "json")]
async fn eventinstances_rocket_get(id: uuid::Uuid) -> rocket::response::content::RawJson<String> {
    let mut client = create_client().await;
    rocket::response::content::RawJson(serde_json::to_string(&eventinstances::select(&mut client, &id).await.unwrap().unwrap()).unwrap())
}

#[rocket::get("/calendarentry/<id>", format = "json")]
async fn calendarentry_rocket_get(id: i64) -> rocket::response::content::RawJson<String> {
    let mut client = create_client().await;
    rocket::response::content::RawJson(serde_json::to_string(&calendarentry::select(&mut client, &id).await.unwrap().unwrap()).unwrap())
}