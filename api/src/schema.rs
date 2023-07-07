pub mod where_types {
    /// used as where clause for type: eventrel 
    ///  Each field that appears in the generated type is wrapped in an Option that determines wether or not it should appear in the where clause, nested options are treated as nullable arguments in the where clause.
    #[derive(Debug, Clone, Serialize, Deserialize, Default)]
    pub struct Whereeventrel {
        pub id: Option<uuid::Uuid>,
        pub source: Option<String>,
        pub key: Option<String>,
        where_kind: WhereType,
    }

    impl Whereeventrel {
        pub fn id(&mut self, id: uuid::Uuid) -> &mut Self {
            self.id = Some(id);
            self
        }

        pub fn source(&mut self, source: String) -> &mut Self {
            self.source = Some(source);
            self
        }

        pub fn key(&mut self, key: String) -> &mut Self {
            self.key = Some(key);
            self
        }

        pub fn operator(&mut self, operator: WhereType) -> &mut Self {
            self.where_kind = operator;
            self
        }
    }

    impl From<super::eventrel> for Whereeventrel {
        fn from(other: super::eventrel) -> Whereeventrel {
            Self {
            id: Some(other.id),
            source: Some(other.source),
            key: Some(other.key),
            where_kind: WhereType::And
            }
        }
    }

    /// used as where clause for type: attendance 
    ///  Each field that appears in the generated type is wrapped in an Option that determines wether or not it should appear in the where clause, nested options are treated as nullable arguments in the where clause.
    #[derive(Debug, Clone, Serialize, Deserialize, Default)]
    pub struct Whereattendance {
        pub participantid: Option<Option<uuid::Uuid>>,
        pub instanceid: Option<Option<uuid::Uuid>>,
        pub id: Option<uuid::Uuid>,
        pub status: Option<Option<String>>,
        where_kind: WhereType,
    }

    impl Whereattendance {
        pub fn participantid(&mut self, participantid: Option<uuid::Uuid>) -> &mut Self {
            self.participantid = Some(participantid);
            self
        }

        pub fn instanceid(&mut self, instanceid: Option<uuid::Uuid>) -> &mut Self {
            self.instanceid = Some(instanceid);
            self
        }

        pub fn id(&mut self, id: uuid::Uuid) -> &mut Self {
            self.id = Some(id);
            self
        }

        pub fn status(&mut self, status: Option<String>) -> &mut Self {
            self.status = Some(status);
            self
        }

        pub fn operator(&mut self, operator: WhereType) -> &mut Self {
            self.where_kind = operator;
            self
        }
    }

    impl From<super::attendance> for Whereattendance {
        fn from(other: super::attendance) -> Whereattendance {
            Self {
            participantid: Some(other.participantid),
            instanceid: Some(other.instanceid),
            id: Some(other.id),
            status: Some(other.status),
            where_kind: WhereType::And
            }
        }
    }

    /// used as where clause for type: participants 
    ///  Each field that appears in the generated type is wrapped in an Option that determines wether or not it should appear in the where clause, nested options are treated as nullable arguments in the where clause.
    #[derive(Debug, Clone, Serialize, Deserialize, Default)]
    pub struct Whereparticipants {
        pub id: Option<uuid::Uuid>,
        pub participant_id: Option<i32>,
        pub first_name: Option<Option<String>>,
        pub last_name: Option<Option<String>>,
        pub email: Option<Option<String>>,
        where_kind: WhereType,
    }

    impl Whereparticipants {
        pub fn id(&mut self, id: uuid::Uuid) -> &mut Self {
            self.id = Some(id);
            self
        }

        pub fn participant_id(&mut self, participant_id: i32) -> &mut Self {
            self.participant_id = Some(participant_id);
            self
        }

        pub fn first_name(&mut self, first_name: Option<String>) -> &mut Self {
            self.first_name = Some(first_name);
            self
        }

        pub fn last_name(&mut self, last_name: Option<String>) -> &mut Self {
            self.last_name = Some(last_name);
            self
        }

        pub fn email(&mut self, email: Option<String>) -> &mut Self {
            self.email = Some(email);
            self
        }

        pub fn operator(&mut self, operator: WhereType) -> &mut Self {
            self.where_kind = operator;
            self
        }
    }

    impl From<super::participants> for Whereparticipants {
        fn from(other: super::participants) -> Whereparticipants {
            Self {
            id: Some(other.id),
            participant_id: Some(other.participant_id),
            first_name: Some(other.first_name),
            last_name: Some(other.last_name),
            email: Some(other.email),
            where_kind: WhereType::And
            }
        }
    }

    /// used as where clause for type: appusageevents 
    ///  Each field that appears in the generated type is wrapped in an Option that determines wether or not it should appear in the where clause, nested options are treated as nullable arguments in the where clause.
    #[derive(Debug, Clone, Serialize, Deserialize, Default)]
    pub struct Whereappusageevents {
        pub metadata_id: Option<Option<uuid::Uuid>>,
        pub platform_type: Option<Option<i32>>,
        pub date: Option<Option<i64>>,
        pub id: Option<uuid::Uuid>,
        pub kind: Option<Option<String>>,
        pub package_name: Option<Option<String>>,
        pub minute: Option<Option<String>>,
        where_kind: WhereType,
    }

    impl Whereappusageevents {
        pub fn metadata_id(&mut self, metadata_id: Option<uuid::Uuid>) -> &mut Self {
            self.metadata_id = Some(metadata_id);
            self
        }

        pub fn platform_type(&mut self, platform_type: Option<i32>) -> &mut Self {
            self.platform_type = Some(platform_type);
            self
        }

        pub fn date(&mut self, date: Option<i64>) -> &mut Self {
            self.date = Some(date);
            self
        }

        pub fn id(&mut self, id: uuid::Uuid) -> &mut Self {
            self.id = Some(id);
            self
        }

        pub fn kind(&mut self, kind: Option<String>) -> &mut Self {
            self.kind = Some(kind);
            self
        }

        pub fn package_name(&mut self, package_name: Option<String>) -> &mut Self {
            self.package_name = Some(package_name);
            self
        }

        pub fn minute(&mut self, minute: Option<String>) -> &mut Self {
            self.minute = Some(minute);
            self
        }

        pub fn operator(&mut self, operator: WhereType) -> &mut Self {
            self.where_kind = operator;
            self
        }
    }

    impl From<super::appusageevents> for Whereappusageevents {
        fn from(other: super::appusageevents) -> Whereappusageevents {
            Self {
            metadata_id: Some(other.metadata_id),
            platform_type: Some(other.platform_type),
            date: Some(other.date),
            id: Some(other.id),
            kind: Some(other.kind),
            package_name: Some(other.package_name),
            minute: Some(other.minute),
            where_kind: WhereType::And
            }
        }
    }

    /// used as where clause for type: events 
    ///  Each field that appears in the generated type is wrapped in an Option that determines wether or not it should appear in the where clause, nested options are treated as nullable arguments in the where clause.
    #[derive(Debug, Clone, Serialize, Deserialize, Default)]
    pub struct Whereevents {
        pub repeats: Option<Option<bool>>,
        pub id: Option<uuid::Uuid>,
        pub organizer: Option<Option<String>>,
        pub description: Option<Option<String>>,
        pub repitition: Option<Option<String>>,
        pub days_of_week: Option<Option<Vec<String>>>,
        pub image_url: Option<Option<String>>,
        pub color: Option<Option<String>>,
        pub image_description: Option<Option<String>>,
        pub title: Option<String>,
        where_kind: WhereType,
    }

    impl Whereevents {
        pub fn repeats(&mut self, repeats: Option<bool>) -> &mut Self {
            self.repeats = Some(repeats);
            self
        }

        pub fn id(&mut self, id: uuid::Uuid) -> &mut Self {
            self.id = Some(id);
            self
        }

        pub fn organizer(&mut self, organizer: Option<String>) -> &mut Self {
            self.organizer = Some(organizer);
            self
        }

        pub fn description(&mut self, description: Option<String>) -> &mut Self {
            self.description = Some(description);
            self
        }

        pub fn repitition(&mut self, repitition: Option<String>) -> &mut Self {
            self.repitition = Some(repitition);
            self
        }

        pub fn days_of_week(&mut self, days_of_week: Option<Vec<String>>) -> &mut Self {
            self.days_of_week = Some(days_of_week);
            self
        }

        pub fn image_url(&mut self, image_url: Option<String>) -> &mut Self {
            self.image_url = Some(image_url);
            self
        }

        pub fn color(&mut self, color: Option<String>) -> &mut Self {
            self.color = Some(color);
            self
        }

        pub fn image_description(&mut self, image_description: Option<String>) -> &mut Self {
            self.image_description = Some(image_description);
            self
        }

        pub fn title(&mut self, title: String) -> &mut Self {
            self.title = Some(title);
            self
        }

        pub fn operator(&mut self, operator: WhereType) -> &mut Self {
            self.where_kind = operator;
            self
        }
    }

    impl From<super::events> for Whereevents {
        fn from(other: super::events) -> Whereevents {
            Self {
            repeats: Some(other.repeats),
            id: Some(other.id),
            organizer: Some(other.organizer),
            description: Some(other.description),
            repitition: Some(other.repitition),
            days_of_week: Some(other.days_of_week),
            image_url: Some(other.image_url),
            color: Some(other.color),
            image_description: Some(other.image_description),
            title: Some(other.title),
            where_kind: WhereType::And
            }
        }
    }

    /// used as where clause for type: smsdata 
    ///  Each field that appears in the generated type is wrapped in an Option that determines wether or not it should appear in the where clause, nested options are treated as nullable arguments in the where clause.
    #[derive(Debug, Clone, Serialize, Deserialize, Default)]
    pub struct Wheresmsdata {
        pub inbound: Option<bool>,
        pub platform_id: Option<Option<i32>>,
        pub date_sent: Option<Option<i64>>,
        pub id: Option<uuid::Uuid>,
        pub date: Option<Option<i64>>,
        pub thread_id: Option<i32>,
        pub metadata_id: Option<Option<uuid::Uuid>>,
        pub minute: Option<Option<String>>,
        pub body: Option<Option<String>>,
        pub address: Option<Option<String>>,
        pub recipient_id: Option<Option<String>>,
        where_kind: WhereType,
    }

    impl Wheresmsdata {
        pub fn inbound(&mut self, inbound: bool) -> &mut Self {
            self.inbound = Some(inbound);
            self
        }

        pub fn platform_id(&mut self, platform_id: Option<i32>) -> &mut Self {
            self.platform_id = Some(platform_id);
            self
        }

        pub fn date_sent(&mut self, date_sent: Option<i64>) -> &mut Self {
            self.date_sent = Some(date_sent);
            self
        }

        pub fn id(&mut self, id: uuid::Uuid) -> &mut Self {
            self.id = Some(id);
            self
        }

        pub fn date(&mut self, date: Option<i64>) -> &mut Self {
            self.date = Some(date);
            self
        }

        pub fn thread_id(&mut self, thread_id: i32) -> &mut Self {
            self.thread_id = Some(thread_id);
            self
        }

        pub fn metadata_id(&mut self, metadata_id: Option<uuid::Uuid>) -> &mut Self {
            self.metadata_id = Some(metadata_id);
            self
        }

        pub fn minute(&mut self, minute: Option<String>) -> &mut Self {
            self.minute = Some(minute);
            self
        }

        pub fn body(&mut self, body: Option<String>) -> &mut Self {
            self.body = Some(body);
            self
        }

        pub fn address(&mut self, address: Option<String>) -> &mut Self {
            self.address = Some(address);
            self
        }

        pub fn recipient_id(&mut self, recipient_id: Option<String>) -> &mut Self {
            self.recipient_id = Some(recipient_id);
            self
        }

        pub fn operator(&mut self, operator: WhereType) -> &mut Self {
            self.where_kind = operator;
            self
        }
    }

    impl From<super::smsdata> for Wheresmsdata {
        fn from(other: super::smsdata) -> Wheresmsdata {
            Self {
            inbound: Some(other.inbound),
            platform_id: Some(other.platform_id),
            date_sent: Some(other.date_sent),
            id: Some(other.id),
            date: Some(other.date),
            thread_id: Some(other.thread_id),
            metadata_id: Some(other.metadata_id),
            minute: Some(other.minute),
            body: Some(other.body),
            address: Some(other.address),
            recipient_id: Some(other.recipient_id),
            where_kind: WhereType::And
            }
        }
    }

    /// used as where clause for type: metadata 
    ///  Each field that appears in the generated type is wrapped in an Option that determines wether or not it should appear in the where clause, nested options are treated as nullable arguments in the where clause.
    #[derive(Debug, Clone, Serialize, Deserialize, Default)]
    pub struct Wheremetadata {
        pub id: Option<uuid::Uuid>,
        pub participant_id: Option<Option<uuid::Uuid>>,
        pub device_id: Option<Option<uuid::Uuid>>,
        where_kind: WhereType,
    }

    impl Wheremetadata {
        pub fn id(&mut self, id: uuid::Uuid) -> &mut Self {
            self.id = Some(id);
            self
        }

        pub fn participant_id(&mut self, participant_id: Option<uuid::Uuid>) -> &mut Self {
            self.participant_id = Some(participant_id);
            self
        }

        pub fn device_id(&mut self, device_id: Option<uuid::Uuid>) -> &mut Self {
            self.device_id = Some(device_id);
            self
        }

        pub fn operator(&mut self, operator: WhereType) -> &mut Self {
            self.where_kind = operator;
            self
        }
    }

    impl From<super::metadata> for Wheremetadata {
        fn from(other: super::metadata) -> Wheremetadata {
            Self {
            id: Some(other.id),
            participant_id: Some(other.participant_id),
            device_id: Some(other.device_id),
            where_kind: WhereType::And
            }
        }
    }

    /// used as where clause for type: eventinstances 
    ///  Each field that appears in the generated type is wrapped in an Option that determines wether or not it should appear in the where clause, nested options are treated as nullable arguments in the where clause.
    #[derive(Debug, Clone, Serialize, Deserialize, Default)]
    pub struct Whereeventinstances {
        pub id: Option<uuid::Uuid>,
        pub event_id: Option<uuid::Uuid>,
        pub begin: Option<i64>,
        pub end: Option<i64>,
        pub duration: Option<Option<i64>>,
        where_kind: WhereType,
    }

    impl Whereeventinstances {
        pub fn id(&mut self, id: uuid::Uuid) -> &mut Self {
            self.id = Some(id);
            self
        }

        pub fn event_id(&mut self, event_id: uuid::Uuid) -> &mut Self {
            self.event_id = Some(event_id);
            self
        }

        pub fn begin(&mut self, begin: i64) -> &mut Self {
            self.begin = Some(begin);
            self
        }

        pub fn end(&mut self, end: i64) -> &mut Self {
            self.end = Some(end);
            self
        }

        pub fn duration(&mut self, duration: Option<i64>) -> &mut Self {
            self.duration = Some(duration);
            self
        }

        pub fn operator(&mut self, operator: WhereType) -> &mut Self {
            self.where_kind = operator;
            self
        }
    }

    impl From<super::eventinstances> for Whereeventinstances {
        fn from(other: super::eventinstances) -> Whereeventinstances {
            Self {
            id: Some(other.id),
            event_id: Some(other.event_id),
            begin: Some(other.begin),
            end: Some(other.end),
            duration: Some(other.duration),
            where_kind: WhereType::And
            }
        }
    }

    /// used as where clause for type: calllog 
    ///  Each field that appears in the generated type is wrapped in an Option that determines wether or not it should appear in the where clause, nested options are treated as nullable arguments in the where clause.
    #[derive(Debug, Clone, Serialize, Deserialize, Default)]
    pub struct Wherecalllog {
        pub id: Option<uuid::Uuid>,
        pub metadata_id: Option<Option<uuid::Uuid>>,
        pub date: Option<Option<i64>>,
        pub duration: Option<Option<i32>>,
        pub recipiend_id: Option<Option<uuid::Uuid>>,
        where_kind: WhereType,
    }

    impl Wherecalllog {
        pub fn id(&mut self, id: uuid::Uuid) -> &mut Self {
            self.id = Some(id);
            self
        }

        pub fn metadata_id(&mut self, metadata_id: Option<uuid::Uuid>) -> &mut Self {
            self.metadata_id = Some(metadata_id);
            self
        }

        pub fn date(&mut self, date: Option<i64>) -> &mut Self {
            self.date = Some(date);
            self
        }

        pub fn duration(&mut self, duration: Option<i32>) -> &mut Self {
            self.duration = Some(duration);
            self
        }

        pub fn recipiend_id(&mut self, recipiend_id: Option<uuid::Uuid>) -> &mut Self {
            self.recipiend_id = Some(recipiend_id);
            self
        }

        pub fn operator(&mut self, operator: WhereType) -> &mut Self {
            self.where_kind = operator;
            self
        }
    }

    impl From<super::calllog> for Wherecalllog {
        fn from(other: super::calllog) -> Wherecalllog {
            Self {
            id: Some(other.id),
            metadata_id: Some(other.metadata_id),
            date: Some(other.date),
            duration: Some(other.duration),
            recipiend_id: Some(other.recipiend_id),
            where_kind: WhereType::And
            }
        }
    }

    #[derive(Debug, Clone, Serialize, Deserialize, Default)]
    pub enum WhereType {
        Or,
        #[default]
        And,
    }
}


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
    "QKzxIc17u80="
}

#[async_trait::async_trait]
/// the primary trait that is used for ORM CRUD operations, ## NOTE: Future plans include a default impl for insert_one, update_one, delete_one, select_one that use self.into::<WhereSelf>()
pub trait DBRow<T>
where Self: Sized,
      T: std::marker::Send,
      Self: std::marker::Sync,
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

    async fn insert_many(client: &mut T, values: &[Self]) -> Result<u64, Self::Err> {
        for value in values.iter() {
             let fut = value.insert(client);
             fut.await?;
        }
        Ok(values.len() as u64)
    }

    async fn update_many(client: &mut T, values: &[Self]) -> Result<u64, Self::Err> {
        for value in values.iter() {
             let fut = value.update(client);
             fut.await?;
        }
        Ok(values.len() as u64)
    }

    /// creates Self based on the primary key value passed into the function
    async fn select_many<P>(client: &mut T, pkeys: &[P]) -> Result<Vec<Option<Self>>, Self::Err>
    where P: tokio_postgres::types::ToSql,
          P: std::marker::Sync,
          P: std::marker::Send,
    {
        let mut outvec = Vec::with_capacity(pkeys.len());
        for value in pkeys.iter() {
             let fut = Self::select(client, value);
             outvec.push(fut.await?);
        }
        Ok(outvec)
    }

    async fn delete_many(client: &mut T, values: &[Self]) -> Result<u64, Self::Err> {
        for value in values.iter() {
             let fut = value.delete(client);
             fut.await?;
        }
        Ok(values.len() as u64)
    }
}

trait SqlCompat {
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
impl DBRow<tokio_postgres::Client> for calllog {
    type Err = tokio_postgres::Error;

    async fn delete(&self, client: &mut tokio_postgres::Client) -> Result<u64, Self::Err> {
        client.execute("delete from calllog where id = $1", &[&self.id]).await
    }

    async fn insert(&self, client: &mut tokio_postgres::Client) -> Result<u64, Self::Err> {
        client.execute("insert into calllog (id, metadata_id, date, duration, recipiend_id) values ($1, $2, $3, $4, $5)", &[&self.id, &self.metadata_id, &self.date, &self.duration, &self.recipiend_id]).await
    }

    /// creates Self based on the primary key value passed into the function
    async fn select<P>(client: &mut tokio_postgres::Client, pkey: &P) -> Result<Option<Self>, Self::Err>
    where P: tokio_postgres::types::ToSql,
          P: std::marker::Sync,
    {
        let selected = client.query("select id, metadata_id, date, duration, recipiend_id from calllog where id = $1", &[pkey]).await?;
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
        client.execute("update calllog set id = $1, metadata_id = $2, date = $3, duration = $4, recipiend_id = $5 where id = $6", &[&self.id, &self.metadata_id, &self.date, &self.duration, &self.recipiend_id, &self.id]).await
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct eventrel {
    pub id: uuid::Uuid,
    pub source: String,
    pub key: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct attendance {
    pub participantid: Option<uuid::Uuid>,
    pub instanceid: Option<uuid::Uuid>,
    pub id: uuid::Uuid,
    pub status: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct participants {
    pub id: uuid::Uuid,
    pub participant_id: i32,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct appusageevents {
    pub metadata_id: Option<uuid::Uuid>,
    pub platform_type: Option<i32>,
    pub date: Option<i64>,
    pub id: uuid::Uuid,
    pub kind: Option<String>,
    pub package_name: Option<String>,
    pub minute: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
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

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
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

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct metadata {
    pub id: uuid::Uuid,
    pub participant_id: Option<uuid::Uuid>,
    pub device_id: Option<uuid::Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct eventinstances {
    pub id: uuid::Uuid,
    pub event_id: uuid::Uuid,
    pub begin: i64,
    pub end: i64,
    pub duration: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct calllog {
    pub id: uuid::Uuid,
    pub metadata_id: Option<uuid::Uuid>,
    pub date: Option<i64>,
    pub duration: Option<i32>,
    pub recipiend_id: Option<uuid::Uuid>,
}

pub fn mount_routes(mut rocket: rocket::Rocket<rocket::Build>) -> rocket::Rocket<rocket::Build> {
    rocket
       .mount("/api/", rocket::routes![eventrel_rocket_get])
       .mount("/api/", rocket::routes![eventrel_rocket_delete])
       .mount("/api/", rocket::routes![eventrel_rocket_post])
       .mount("/api/", rocket::routes![eventrel_rocket_put])
       .mount("/api/", rocket::routes![attendance_rocket_get])
       .mount("/api/", rocket::routes![attendance_rocket_delete])
       .mount("/api/", rocket::routes![attendance_rocket_post])
       .mount("/api/", rocket::routes![attendance_rocket_put])
       .mount("/api/", rocket::routes![participants_rocket_get])
       .mount("/api/", rocket::routes![participants_rocket_delete])
       .mount("/api/", rocket::routes![participants_rocket_post])
       .mount("/api/", rocket::routes![participants_rocket_put])
       .mount("/api/", rocket::routes![appusageevents_rocket_get])
       .mount("/api/", rocket::routes![appusageevents_rocket_delete])
       .mount("/api/", rocket::routes![appusageevents_rocket_post])
       .mount("/api/", rocket::routes![appusageevents_rocket_put])
       .mount("/api/", rocket::routes![events_rocket_get])
       .mount("/api/", rocket::routes![events_rocket_delete])
       .mount("/api/", rocket::routes![events_rocket_post])
       .mount("/api/", rocket::routes![events_rocket_put])
       .mount("/api/", rocket::routes![smsdata_rocket_get])
       .mount("/api/", rocket::routes![smsdata_rocket_delete])
       .mount("/api/", rocket::routes![smsdata_rocket_post])
       .mount("/api/", rocket::routes![smsdata_rocket_put])
       .mount("/api/", rocket::routes![metadata_rocket_get])
       .mount("/api/", rocket::routes![metadata_rocket_delete])
       .mount("/api/", rocket::routes![metadata_rocket_post])
       .mount("/api/", rocket::routes![metadata_rocket_put])
       .mount("/api/", rocket::routes![eventinstances_rocket_get])
       .mount("/api/", rocket::routes![eventinstances_rocket_delete])
       .mount("/api/", rocket::routes![eventinstances_rocket_post])
       .mount("/api/", rocket::routes![eventinstances_rocket_put])
       .mount("/api/", rocket::routes![calllog_rocket_get])
       .mount("/api/", rocket::routes![calllog_rocket_delete])
       .mount("/api/", rocket::routes![calllog_rocket_post])
       .mount("/api/", rocket::routes![calllog_rocket_put])
}

type eventrel_pkey_list = Vec<uuid::Uuid>;
#[rocket::post("/eventrel/instance/", format = "json", data = "<eventrel>")]
async fn eventrel_rocket_post(eventrel: rocket::serde::json::Json<eventrel>) {
    let mut client = create_client().await;
    eventrel.0.insert(&mut client).await.unwrap();
}

#[rocket::get("/eventrel/by-id/<id>", format = "json")]
async fn eventrel_rocket_get(id: uuid::Uuid) -> rocket::response::content::RawJson<String> {
    let mut client = create_client().await;
    rocket::response::content::RawJson(serde_json::to_string(&eventrel::select(&mut client, &id).await.unwrap().unwrap()).unwrap())
}

#[rocket::delete("/eventrel/by-id/<id>")]
async fn eventrel_rocket_delete(id: uuid::Uuid) -> rocket::response::content::RawText<&'static str> {
    let mut client = create_client().await;
    let mut todelete = eventrel::default();
    todelete.id = id;
    todelete.delete(&mut client).await.unwrap();
    rocket::response::content::RawText("it worked")
}

#[rocket::put("/eventrel/instance/", format = "json", data = "<eventrel>")]
async fn eventrel_rocket_put(eventrel: rocket::serde::json::Json<eventrel>) {
    let mut client = create_client().await;
    eventrel.0.update(&mut client).await.unwrap();
}

type attendance_pkey_list = Vec<uuid::Uuid>;
#[rocket::post("/attendance/instance/", format = "json", data = "<attendance>")]
async fn attendance_rocket_post(attendance: rocket::serde::json::Json<attendance>) {
    let mut client = create_client().await;
    attendance.0.insert(&mut client).await.unwrap();
}

#[rocket::get("/attendance/by-id/<id>", format = "json")]
async fn attendance_rocket_get(id: uuid::Uuid) -> rocket::response::content::RawJson<String> {
    let mut client = create_client().await;
    rocket::response::content::RawJson(serde_json::to_string(&attendance::select(&mut client, &id).await.unwrap().unwrap()).unwrap())
}

#[rocket::delete("/attendance/by-id/<id>")]
async fn attendance_rocket_delete(id: uuid::Uuid) -> rocket::response::content::RawText<&'static str> {
    let mut client = create_client().await;
    let mut todelete = attendance::default();
    todelete.id = id;
    todelete.delete(&mut client).await.unwrap();
    rocket::response::content::RawText("it worked")
}

#[rocket::put("/attendance/instance/", format = "json", data = "<attendance>")]
async fn attendance_rocket_put(attendance: rocket::serde::json::Json<attendance>) {
    let mut client = create_client().await;
    attendance.0.update(&mut client).await.unwrap();
}

type participants_pkey_list = Vec<uuid::Uuid>;
#[rocket::post("/participants/instance/", format = "json", data = "<participants>")]
async fn participants_rocket_post(participants: rocket::serde::json::Json<participants>) {
    let mut client = create_client().await;
    participants.0.insert(&mut client).await.unwrap();
}

#[rocket::get("/participants/by-id/<id>", format = "json")]
async fn participants_rocket_get(id: uuid::Uuid) -> rocket::response::content::RawJson<String> {
    let mut client = create_client().await;
    rocket::response::content::RawJson(serde_json::to_string(&participants::select(&mut client, &id).await.unwrap().unwrap()).unwrap())
}

#[rocket::delete("/participants/by-id/<id>")]
async fn participants_rocket_delete(id: uuid::Uuid) -> rocket::response::content::RawText<&'static str> {
    let mut client = create_client().await;
    let mut todelete = participants::default();
    todelete.id = id;
    todelete.delete(&mut client).await.unwrap();
    rocket::response::content::RawText("it worked")
}

#[rocket::put("/participants/instance/", format = "json", data = "<participants>")]
async fn participants_rocket_put(participants: rocket::serde::json::Json<participants>) {
    let mut client = create_client().await;
    participants.0.update(&mut client).await.unwrap();
}

type appusageevents_pkey_list = Vec<uuid::Uuid>;
#[rocket::post("/appusageevents/instance/", format = "json", data = "<appusageevents>")]
async fn appusageevents_rocket_post(appusageevents: rocket::serde::json::Json<appusageevents>) {
    let mut client = create_client().await;
    appusageevents.0.insert(&mut client).await.unwrap();
}

#[rocket::get("/appusageevents/by-id/<id>", format = "json")]
async fn appusageevents_rocket_get(id: uuid::Uuid) -> rocket::response::content::RawJson<String> {
    let mut client = create_client().await;
    rocket::response::content::RawJson(serde_json::to_string(&appusageevents::select(&mut client, &id).await.unwrap().unwrap()).unwrap())
}

#[rocket::delete("/appusageevents/by-id/<id>")]
async fn appusageevents_rocket_delete(id: uuid::Uuid) -> rocket::response::content::RawText<&'static str> {
    let mut client = create_client().await;
    let mut todelete = appusageevents::default();
    todelete.id = id;
    todelete.delete(&mut client).await.unwrap();
    rocket::response::content::RawText("it worked")
}

#[rocket::put("/appusageevents/instance/", format = "json", data = "<appusageevents>")]
async fn appusageevents_rocket_put(appusageevents: rocket::serde::json::Json<appusageevents>) {
    let mut client = create_client().await;
    appusageevents.0.update(&mut client).await.unwrap();
}

type events_pkey_list = Vec<uuid::Uuid>;
#[rocket::post("/events/instance/", format = "json", data = "<events>")]
async fn events_rocket_post(events: rocket::serde::json::Json<events>) {
    let mut client = create_client().await;
    events.0.insert(&mut client).await.unwrap();
}

#[rocket::get("/events/by-id/<id>", format = "json")]
async fn events_rocket_get(id: uuid::Uuid) -> rocket::response::content::RawJson<String> {
    let mut client = create_client().await;
    rocket::response::content::RawJson(serde_json::to_string(&events::select(&mut client, &id).await.unwrap().unwrap()).unwrap())
}

#[rocket::delete("/events/by-id/<id>")]
async fn events_rocket_delete(id: uuid::Uuid) -> rocket::response::content::RawText<&'static str> {
    let mut client = create_client().await;
    let mut todelete = events::default();
    todelete.id = id;
    todelete.delete(&mut client).await.unwrap();
    rocket::response::content::RawText("it worked")
}

#[rocket::put("/events/instance/", format = "json", data = "<events>")]
async fn events_rocket_put(events: rocket::serde::json::Json<events>) {
    let mut client = create_client().await;
    events.0.update(&mut client).await.unwrap();
}

type smsdata_pkey_list = Vec<uuid::Uuid>;
#[rocket::post("/smsdata/instance/", format = "json", data = "<smsdata>")]
async fn smsdata_rocket_post(smsdata: rocket::serde::json::Json<smsdata>) {
    let mut client = create_client().await;
    smsdata.0.insert(&mut client).await.unwrap();
}

#[rocket::get("/smsdata/by-id/<id>", format = "json")]
async fn smsdata_rocket_get(id: uuid::Uuid) -> rocket::response::content::RawJson<String> {
    let mut client = create_client().await;
    rocket::response::content::RawJson(serde_json::to_string(&smsdata::select(&mut client, &id).await.unwrap().unwrap()).unwrap())
}

#[rocket::delete("/smsdata/by-id/<id>")]
async fn smsdata_rocket_delete(id: uuid::Uuid) -> rocket::response::content::RawText<&'static str> {
    let mut client = create_client().await;
    let mut todelete = smsdata::default();
    todelete.id = id;
    todelete.delete(&mut client).await.unwrap();
    rocket::response::content::RawText("it worked")
}

#[rocket::put("/smsdata/instance/", format = "json", data = "<smsdata>")]
async fn smsdata_rocket_put(smsdata: rocket::serde::json::Json<smsdata>) {
    let mut client = create_client().await;
    smsdata.0.update(&mut client).await.unwrap();
}

type metadata_pkey_list = Vec<uuid::Uuid>;
#[rocket::post("/metadata/instance/", format = "json", data = "<metadata>")]
async fn metadata_rocket_post(metadata: rocket::serde::json::Json<metadata>) {
    let mut client = create_client().await;
    metadata.0.insert(&mut client).await.unwrap();
}

#[rocket::get("/metadata/by-id/<id>", format = "json")]
async fn metadata_rocket_get(id: uuid::Uuid) -> rocket::response::content::RawJson<String> {
    let mut client = create_client().await;
    rocket::response::content::RawJson(serde_json::to_string(&metadata::select(&mut client, &id).await.unwrap().unwrap()).unwrap())
}

#[rocket::delete("/metadata/by-id/<id>")]
async fn metadata_rocket_delete(id: uuid::Uuid) -> rocket::response::content::RawText<&'static str> {
    let mut client = create_client().await;
    let mut todelete = metadata::default();
    todelete.id = id;
    todelete.delete(&mut client).await.unwrap();
    rocket::response::content::RawText("it worked")
}

#[rocket::put("/metadata/instance/", format = "json", data = "<metadata>")]
async fn metadata_rocket_put(metadata: rocket::serde::json::Json<metadata>) {
    let mut client = create_client().await;
    metadata.0.update(&mut client).await.unwrap();
}

type eventinstances_pkey_list = Vec<uuid::Uuid>;
#[rocket::post("/eventinstances/instance/", format = "json", data = "<eventinstances>")]
async fn eventinstances_rocket_post(eventinstances: rocket::serde::json::Json<eventinstances>) {
    let mut client = create_client().await;
    eventinstances.0.insert(&mut client).await.unwrap();
}

#[rocket::get("/eventinstances/by-id/<id>", format = "json")]
async fn eventinstances_rocket_get(id: uuid::Uuid) -> rocket::response::content::RawJson<String> {
    let mut client = create_client().await;
    rocket::response::content::RawJson(serde_json::to_string(&eventinstances::select(&mut client, &id).await.unwrap().unwrap()).unwrap())
}

#[rocket::delete("/eventinstances/by-id/<id>")]
async fn eventinstances_rocket_delete(id: uuid::Uuid) -> rocket::response::content::RawText<&'static str> {
    let mut client = create_client().await;
    let mut todelete = eventinstances::default();
    todelete.id = id;
    todelete.delete(&mut client).await.unwrap();
    rocket::response::content::RawText("it worked")
}

#[rocket::put("/eventinstances/instance/", format = "json", data = "<eventinstances>")]
async fn eventinstances_rocket_put(eventinstances: rocket::serde::json::Json<eventinstances>) {
    let mut client = create_client().await;
    eventinstances.0.update(&mut client).await.unwrap();
}

type calllog_pkey_list = Vec<uuid::Uuid>;
#[rocket::post("/calllog/instance/", format = "json", data = "<calllog>")]
async fn calllog_rocket_post(calllog: rocket::serde::json::Json<calllog>) {
    let mut client = create_client().await;
    calllog.0.insert(&mut client).await.unwrap();
}

#[rocket::get("/calllog/by-id/<id>", format = "json")]
async fn calllog_rocket_get(id: uuid::Uuid) -> rocket::response::content::RawJson<String> {
    let mut client = create_client().await;
    rocket::response::content::RawJson(serde_json::to_string(&calllog::select(&mut client, &id).await.unwrap().unwrap()).unwrap())
}

#[rocket::delete("/calllog/by-id/<id>")]
async fn calllog_rocket_delete(id: uuid::Uuid) -> rocket::response::content::RawText<&'static str> {
    let mut client = create_client().await;
    let mut todelete = calllog::default();
    todelete.id = id;
    todelete.delete(&mut client).await.unwrap();
    rocket::response::content::RawText("it worked")
}

#[rocket::put("/calllog/instance/", format = "json", data = "<calllog>")]
async fn calllog_rocket_put(calllog: rocket::serde::json::Json<calllog>) {
    let mut client = create_client().await;
    calllog.0.update(&mut client).await.unwrap();
}