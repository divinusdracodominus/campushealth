#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct eventrel {
    pub id: uuid::Uuid,
    pub source: String,
    pub key: String,
}

impl<T> DBRow<T, where_types::Whereeventrel> for eventrel
where T: pgcodegen::db::SimpleClient,
{
    type Err = T::Err;

    fn delete(&self, mut client: T) -> Result<u64, Self::Err> {
        client.exec("delete from eventrel where id = $1", &[&self.id])
    }

    fn insert(&self, mut client: T) -> Result<u64, Self::Err> {
        client.exec("insert into eventrel (id, source, key) values ($1, $2, $3)", &[&self.id, &self.source, &self.key])
    }

    fn select<P>(mut client: T, pkey: &P) -> Result<Vec<Self>, Self::Err>
    where P: postgres_types::ToSql,
    {
        client.exec("select from eventrel where id = $1", &[pkey])
    }

    fn update(&self, mut client: T) -> Result<u64, Self::Err> {
        client.exec("update eventrel set id = $1, source = $2, key = $3 where id = $4", &[&self.id, &self.source, &self.key, &self.id])
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct attendance {
    pub participantid: Option<uuid::Uuid>,
    pub instanceid: Option<uuid::Uuid>,
    pub id: uuid::Uuid,
    pub status: Option<String>,
}

impl<T> DBRow<T, where_types::Whereattendance> for attendance
where T: pgcodegen::db::SimpleClient,
{
    type Err = T::Err;

    fn delete(&self, mut client: T) -> Result<u64, Self::Err> {
        client.exec("delete from attendance where id = $1", &[&self.id])
    }

    fn insert(&self, mut client: T) -> Result<u64, Self::Err> {
        client.exec("insert into attendance (participantid, instanceid, id, status) values ($1, $2, $3, $4)", &[&self.participantid, &self.instanceid, &self.id, &self.status])
    }

    fn select<P>(mut client: T, pkey: &P) -> Result<Vec<Self>, Self::Err>
    where P: postgres_types::ToSql,
    {
        client.exec("select from attendance where id = $1", &[pkey])
    }

    fn update(&self, mut client: T) -> Result<u64, Self::Err> {
        client.exec("update attendance set participantid = $1, instanceid = $2, id = $3, status = $4 where id = $5", &[&self.participantid, &self.instanceid, &self.id, &self.status, &self.id])
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct participants {
    pub id: uuid::Uuid,
    pub participant_id: i32,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: Option<String>,
}

impl<T> DBRow<T, where_types::Whereparticipants> for participants
where T: pgcodegen::db::SimpleClient,
{
    type Err = T::Err;

    fn delete(&self, mut client: T) -> Result<u64, Self::Err> {
        client.exec("delete from participants where id = $1", &[&self.id])
    }

    fn insert(&self, mut client: T) -> Result<u64, Self::Err> {
        client.exec("insert into participants (id, participant_id, first_name, last_name, email) values ($1, $2, $3, $4, $5)", &[&self.id, &self.participant_id, &self.first_name, &self.last_name, &self.email])
    }

    fn select<P>(mut client: T, pkey: &P) -> Result<Vec<Self>, Self::Err>
    where P: postgres_types::ToSql,
    {
        client.exec("select from participants where id = $1", &[pkey])
    }

    fn update(&self, mut client: T) -> Result<u64, Self::Err> {
        client.exec("update participants set id = $1, participant_id = $2, first_name = $3, last_name = $4, email = $5 where id = $6", &[&self.id, &self.participant_id, &self.first_name, &self.last_name, &self.email, &self.id])
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct calendarinstance {
    pub id: i32,
    pub calendar_id: i32,
    pub begin: i64,
    pub end: i64,
}

impl<T> DBRow<T, where_types::Wherecalendarinstance> for calendarinstance
where T: pgcodegen::db::SimpleClient,
{
    type Err = T::Err;

    fn delete(&self, mut client: T) -> Result<u64, Self::Err> {
        client.exec("delete from calendarinstance where id = $1", &[&self.id])
    }

    fn insert(&self, mut client: T) -> Result<u64, Self::Err> {
        client.exec("insert into calendarinstance (id, calendar_id, begin, end) values ($1, $2, $3, $4)", &[&self.id, &self.calendar_id, &self.begin, &self.end])
    }

    fn select<P>(mut client: T, pkey: &P) -> Result<Vec<Self>, Self::Err>
    where P: postgres_types::ToSql,
    {
        client.exec("select from calendarinstance where id = $1", &[pkey])
    }

    fn update(&self, mut client: T) -> Result<u64, Self::Err> {
        client.exec("update calendarinstance set id = $1, calendar_id = $2, begin = $3, end = $4 where id = $5", &[&self.id, &self.calendar_id, &self.begin, &self.end, &self.id])
    }
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

impl<T> DBRow<T, where_types::Whereappusageevents> for appusageevents
where T: pgcodegen::db::SimpleClient,
{
    type Err = T::Err;

    fn delete(&self, mut client: T) -> Result<u64, Self::Err> {
        client.exec("delete from appusageevents where id = $1", &[&self.id])
    }

    fn insert(&self, mut client: T) -> Result<u64, Self::Err> {
        client.exec("insert into appusageevents (metadata_id, platform_type, date, id, kind, package_name, minute) values ($1, $2, $3, $4, $5, $6, $7)", &[&self.metadata_id, &self.platform_type, &self.date, &self.id, &self.kind, &self.package_name, &self.minute])
    }

    fn select<P>(mut client: T, pkey: &P) -> Result<Vec<Self>, Self::Err>
    where P: postgres_types::ToSql,
    {
        client.exec("select from appusageevents where id = $1", &[pkey])
    }

    fn update(&self, mut client: T) -> Result<u64, Self::Err> {
        client.exec("update appusageevents set metadata_id = $1, platform_type = $2, date = $3, id = $4, kind = $5, package_name = $6, minute = $7 where id = $8", &[&self.metadata_id, &self.platform_type, &self.date, &self.id, &self.kind, &self.package_name, &self.minute, &self.id])
    }
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

impl<T> DBRow<T, where_types::Whereevents> for events
where T: pgcodegen::db::SimpleClient,
{
    type Err = T::Err;

    fn delete(&self, mut client: T) -> Result<u64, Self::Err> {
        client.exec("delete from events where id = $1", &[&self.id])
    }

    fn insert(&self, mut client: T) -> Result<u64, Self::Err> {
        client.exec("insert into events (repeats, id, organizer, description, repitition, days_of_week, image_url, color, image_description, title) values ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)", &[&self.repeats, &self.id, &self.organizer, &self.description, &self.repitition, &self.days_of_week, &self.image_url, &self.color, &self.image_description, &self.title])
    }

    fn select<P>(mut client: T, pkey: &P) -> Result<Vec<Self>, Self::Err>
    where P: postgres_types::ToSql,
    {
        client.exec("select from events where id = $1", &[pkey])
    }

    fn update(&self, mut client: T) -> Result<u64, Self::Err> {
        client.exec("update events set repeats = $1, id = $2, organizer = $3, description = $4, repitition = $5, days_of_week = $6, image_url = $7, color = $8, image_description = $9, title = $10 where id = $11", &[&self.repeats, &self.id, &self.organizer, &self.description, &self.repitition, &self.days_of_week, &self.image_url, &self.color, &self.image_description, &self.title, &self.id])
    }
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

impl<T> DBRow<T, where_types::Wheresmsdata> for smsdata
where T: pgcodegen::db::SimpleClient,
{
    type Err = T::Err;

    fn delete(&self, mut client: T) -> Result<u64, Self::Err> {
        client.exec("delete from smsdata where id = $1", &[&self.id])
    }

    fn insert(&self, mut client: T) -> Result<u64, Self::Err> {
        client.exec("insert into smsdata (inbound, platform_id, date_sent, id, date, thread_id, metadata_id, minute, body, address, recipient_id) values ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)", &[&self.inbound, &self.platform_id, &self.date_sent, &self.id, &self.date, &self.thread_id, &self.metadata_id, &self.minute, &self.body, &self.address, &self.recipient_id])
    }

    fn select<P>(mut client: T, pkey: &P) -> Result<Vec<Self>, Self::Err>
    where P: postgres_types::ToSql,
    {
        client.exec("select from smsdata where id = $1", &[pkey])
    }

    fn update(&self, mut client: T) -> Result<u64, Self::Err> {
        client.exec("update smsdata set inbound = $1, platform_id = $2, date_sent = $3, id = $4, date = $5, thread_id = $6, metadata_id = $7, minute = $8, body = $9, address = $10, recipient_id = $11 where id = $12", &[&self.inbound, &self.platform_id, &self.date_sent, &self.id, &self.date, &self.thread_id, &self.metadata_id, &self.minute, &self.body, &self.address, &self.recipient_id, &self.id])
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct calldata {
    pub id: uuid::Uuid,
    pub metadata_id: Option<uuid::Uuid>,
    pub date: Option<i64>,
    pub duration: Option<i32>,
    pub recipiend_id: Option<uuid::Uuid>,
}

impl<T> DBRow<T, where_types::Wherecalldata> for calldata
where T: pgcodegen::db::SimpleClient,
{
    type Err = T::Err;

    fn delete(&self, mut client: T) -> Result<u64, Self::Err> {
        client.exec("delete from calldata where id = $1", &[&self.id])
    }

    fn insert(&self, mut client: T) -> Result<u64, Self::Err> {
        client.exec("insert into calldata (id, metadata_id, date, duration, recipiend_id) values ($1, $2, $3, $4, $5)", &[&self.id, &self.metadata_id, &self.date, &self.duration, &self.recipiend_id])
    }

    fn select<P>(mut client: T, pkey: &P) -> Result<Vec<Self>, Self::Err>
    where P: postgres_types::ToSql,
    {
        client.exec("select from calldata where id = $1", &[pkey])
    }

    fn update(&self, mut client: T) -> Result<u64, Self::Err> {
        client.exec("update calldata set id = $1, metadata_id = $2, date = $3, duration = $4, recipiend_id = $5 where id = $6", &[&self.id, &self.metadata_id, &self.date, &self.duration, &self.recipiend_id, &self.id])
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct metadata {
    pub id: uuid::Uuid,
    pub participant_id: Option<uuid::Uuid>,
    pub device_id: Option<uuid::Uuid>,
}

impl<T> DBRow<T, where_types::Wheremetadata> for metadata
where T: pgcodegen::db::SimpleClient,
{
    type Err = T::Err;

    fn delete(&self, mut client: T) -> Result<u64, Self::Err> {
        client.exec("delete from metadata where id = $1", &[&self.id])
    }

    fn insert(&self, mut client: T) -> Result<u64, Self::Err> {
        client.exec("insert into metadata (id, participant_id, device_id) values ($1, $2, $3)", &[&self.id, &self.participant_id, &self.device_id])
    }

    fn select<P>(mut client: T, pkey: &P) -> Result<Vec<Self>, Self::Err>
    where P: postgres_types::ToSql,
    {
        client.exec("select from metadata where id = $1", &[pkey])
    }

    fn update(&self, mut client: T) -> Result<u64, Self::Err> {
        client.exec("update metadata set id = $1, participant_id = $2, device_id = $3 where id = $4", &[&self.id, &self.participant_id, &self.device_id, &self.id])
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct eventinstances {
    pub id: uuid::Uuid,
    pub event_id: uuid::Uuid,
    pub begin: i64,
    pub end: i64,
    pub duration: Option<i64>,
}

impl<T> DBRow<T, where_types::Whereeventinstances> for eventinstances
where T: pgcodegen::db::SimpleClient,
{
    type Err = T::Err;

    fn delete(&self, mut client: T) -> Result<u64, Self::Err> {
        client.exec("delete from eventinstances where id = $1", &[&self.id])
    }

    fn insert(&self, mut client: T) -> Result<u64, Self::Err> {
        client.exec("insert into eventinstances (id, event_id, begin, end, duration) values ($1, $2, $3, $4, $5)", &[&self.id, &self.event_id, &self.begin, &self.end, &self.duration])
    }

    fn select<P>(mut client: T, pkey: &P) -> Result<Vec<Self>, Self::Err>
    where P: postgres_types::ToSql,
    {
        client.exec("select from eventinstances where id = $1", &[pkey])
    }

    fn update(&self, mut client: T) -> Result<u64, Self::Err> {
        client.exec("update eventinstances set id = $1, event_id = $2, begin = $3, end = $4, duration = $5 where id = $6", &[&self.id, &self.event_id, &self.begin, &self.end, &self.duration, &self.id])
    }
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

impl<T> DBRow<T, where_types::Wherecalendarentry> for calendarentry
where T: pgcodegen::db::SimpleClient,
{
    type Err = T::Err;

    fn delete(&self, mut client: T) -> Result<u64, Self::Err> {
        client.exec("delete from calendarentry where id = $1", &[&self.id])
    }

    fn insert(&self, mut client: T) -> Result<u64, Self::Err> {
        client.exec("insert into calendarentry (metadata, id, selfattendeestatus, eventlocation, owneraccount, title, description) values ($1, $2, $3, $4, $5, $6, $7)", &[&self.metadata, &self.id, &self.selfattendeestatus, &self.eventlocation, &self.owneraccount, &self.title, &self.description])
    }

    fn select<P>(mut client: T, pkey: &P) -> Result<Vec<Self>, Self::Err>
    where P: postgres_types::ToSql,
    {
        client.exec("select from calendarentry where id = $1", &[pkey])
    }

    fn update(&self, mut client: T) -> Result<u64, Self::Err> {
        client.exec("update calendarentry set metadata = $1, id = $2, selfattendeestatus = $3, eventlocation = $4, owneraccount = $5, title = $6, description = $7 where id = $8", &[&self.metadata, &self.id, &self.selfattendeestatus, &self.eventlocation, &self.owneraccount, &self.title, &self.description, &self.id])
    }
}

pub mod where_types {
    /// used as where clause for type: eventrel 
    ///  Each field that appears in the generated type is wrapped in an Option that determines wether or not it should appear in the where clause, nested options are treated as nullable arguments in the where clause.
    #[derive(Debug, Clone, Serialize, Deserialize)]
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
    #[derive(Debug, Clone, Serialize, Deserialize)]
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
    #[derive(Debug, Clone, Serialize, Deserialize)]
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

    /// used as where clause for type: calendarinstance 
    ///  Each field that appears in the generated type is wrapped in an Option that determines wether or not it should appear in the where clause, nested options are treated as nullable arguments in the where clause.
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Wherecalendarinstance {
        pub id: Option<i32>,
        pub calendar_id: Option<i32>,
        pub begin: Option<i64>,
        pub end: Option<i64>,
        where_kind: WhereType,
    }

    impl Wherecalendarinstance {
        pub fn id(&mut self, id: i32) -> &mut Self {
            self.id = Some(id);
            self
        }

        pub fn calendar_id(&mut self, calendar_id: i32) -> &mut Self {
            self.calendar_id = Some(calendar_id);
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

        pub fn operator(&mut self, operator: WhereType) -> &mut Self {
            self.where_kind = operator;
            self
        }
    }

    impl From<super::calendarinstance> for Wherecalendarinstance {
        fn from(other: super::calendarinstance) -> Wherecalendarinstance {
            Self {
            id: Some(other.id),
            calendar_id: Some(other.calendar_id),
            begin: Some(other.begin),
            end: Some(other.end),
            where_kind: WhereType::And
            }
        }
    }

    /// used as where clause for type: appusageevents 
    ///  Each field that appears in the generated type is wrapped in an Option that determines wether or not it should appear in the where clause, nested options are treated as nullable arguments in the where clause.
    #[derive(Debug, Clone, Serialize, Deserialize)]
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
    #[derive(Debug, Clone, Serialize, Deserialize)]
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
    #[derive(Debug, Clone, Serialize, Deserialize)]
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

    /// used as where clause for type: calldata 
    ///  Each field that appears in the generated type is wrapped in an Option that determines wether or not it should appear in the where clause, nested options are treated as nullable arguments in the where clause.
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Wherecalldata {
        pub id: Option<uuid::Uuid>,
        pub metadata_id: Option<Option<uuid::Uuid>>,
        pub date: Option<Option<i64>>,
        pub duration: Option<Option<i32>>,
        pub recipiend_id: Option<Option<uuid::Uuid>>,
        where_kind: WhereType,
    }

    impl Wherecalldata {
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

    impl From<super::calldata> for Wherecalldata {
        fn from(other: super::calldata) -> Wherecalldata {
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

    /// used as where clause for type: metadata 
    ///  Each field that appears in the generated type is wrapped in an Option that determines wether or not it should appear in the where clause, nested options are treated as nullable arguments in the where clause.
    #[derive(Debug, Clone, Serialize, Deserialize)]
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
    #[derive(Debug, Clone, Serialize, Deserialize)]
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

    /// used as where clause for type: calendarentry 
    ///  Each field that appears in the generated type is wrapped in an Option that determines wether or not it should appear in the where clause, nested options are treated as nullable arguments in the where clause.
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Wherecalendarentry {
        pub metadata: Option<Option<uuid::Uuid>>,
        pub id: Option<i64>,
        pub selfattendeestatus: Option<Option<bool>>,
        pub eventlocation: Option<Option<String>>,
        pub owneraccount: Option<Option<String>>,
        pub title: Option<String>,
        pub description: Option<Option<String>>,
        where_kind: WhereType,
    }

    impl Wherecalendarentry {
        pub fn metadata(&mut self, metadata: Option<uuid::Uuid>) -> &mut Self {
            self.metadata = Some(metadata);
            self
        }

        pub fn id(&mut self, id: i64) -> &mut Self {
            self.id = Some(id);
            self
        }

        pub fn selfattendeestatus(&mut self, selfattendeestatus: Option<bool>) -> &mut Self {
            self.selfattendeestatus = Some(selfattendeestatus);
            self
        }

        pub fn eventlocation(&mut self, eventlocation: Option<String>) -> &mut Self {
            self.eventlocation = Some(eventlocation);
            self
        }

        pub fn owneraccount(&mut self, owneraccount: Option<String>) -> &mut Self {
            self.owneraccount = Some(owneraccount);
            self
        }

        pub fn title(&mut self, title: String) -> &mut Self {
            self.title = Some(title);
            self
        }

        pub fn description(&mut self, description: Option<String>) -> &mut Self {
            self.description = Some(description);
            self
        }

        pub fn operator(&mut self, operator: WhereType) -> &mut Self {
            self.where_kind = operator;
            self
        }
    }

    impl From<super::calendarentry> for Wherecalendarentry {
        fn from(other: super::calendarentry) -> Wherecalendarentry {
            Self {
            metadata: Some(other.metadata),
            id: Some(other.id),
            selfattendeestatus: Some(other.selfattendeestatus),
            eventlocation: Some(other.eventlocation),
            owneraccount: Some(other.owneraccount),
            title: Some(other.title),
            description: Some(other.description),
            where_kind: WhereType::And
            }
        }
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub enum WhereType {
        Or,
        #[default]
        And,
    }
}

/// base64 encoded hash of table names, column names, and column types using alphanumerics, ~, _
pub fn version_hash() -> &'static str {
    "n19_f0FTKzs="
}

/// the primary trait that is used for ORM CRUD operations, ## NOTE: Future plans include a default impl for insert_one, update_one, delete_one, select_one that use self.into::<WhereSelf>()
pub trait DBRow<T, W>
where Self: Sized,
{
    type Err: std::error::Error;

    fn delete(&self, client: T) -> Result<u64, Self::Err>;

    fn insert(&self, client: T) -> Result<u64, Self::Err>;

    fn select<P>(client: T, pkey: &P) -> Result<Vec<Self>, Self::Err>
    where P: postgres_types::ToSql,
    ;

    fn update(&self, client: T) -> Result<u64, Self::Err>;
}
