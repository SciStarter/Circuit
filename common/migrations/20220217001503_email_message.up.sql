create table c_email_message (
       slug text not null,
       subject text not null,
       body text not null
);

create unique index c_email_message_by_slug on c_email_message(slug);
