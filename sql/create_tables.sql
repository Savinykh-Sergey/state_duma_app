CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

create table members
(
    id          uuid         not null
        primary key,
    full_name   varchar(100) not null,
    birthday    date         not null,
    experience  integer      not null
        constraint members_experience_check
            check (experience >= 0),
    phone       varchar(20)  not null,
    created_at  timestamp,
    updated_at  timestamp,
    is_chairman boolean
);

alter table members
    owner to admin;

create index idx_members_experience
    on members (experience);

create index idx_members_created_at
    on members (created_at);

create index idx_members_full_name
    on members (full_name);

create trigger trg_check_chairman_experience
    before insert or update
                         on members
                         for each row
                         execute procedure check_chairman_experience();

create table organizers
(
    id         uuid         not null
        primary key,
    full_name  varchar(100) not null,
    phone      varchar(20)  not null,
    birthday   date         not null,
    created_at timestamp,
    updated_at timestamp
);

alter table organizers
    owner to admin;

create index idx_organizers_id
    on organizers (id);

create index idx_organizers_full_name
    on organizers (full_name);

create index idx_organizers_created_at
    on organizers (created_at);

create table commissions
(
    id           uuid                 not null
        primary key,
    direction    commission_direction not null,
    organizer_id uuid                 not null
        references organizers,
    chairman_id  uuid                 not null
        references members,
    created_at   timestamp,
    updated_at   timestamp
);

alter table commissions
    owner to admin;

create index idx_commissions_chairman_id
    on commissions (chairman_id);

create index idx_commissions_organizer_id
    on commissions (organizer_id);

create index idx_commissions_created_at
    on commissions (created_at);

create trigger check_commission_chairman
    before insert or update
                         on commissions
                         for each row
                         execute procedure validate_commission_chairman();

create trigger validate_unique_chairman_trigger
    before insert or update
                         on commissions
                         for each row
                         execute procedure validate_unique_chairman_per_commission();

create table meetings
(
    id            uuid    not null
        primary key,
    date          date    not null,
    duration      integer not null,
    commission_id uuid    not null
        references commissions,
    created_at    timestamp,
    updated_at    timestamp
);

alter table meetings
    owner to admin;

create index idx_meetings_commission_id
    on meetings (commission_id);

create index idx_meetings_date
    on meetings (date);

create index idx_meetings_created_at
    on meetings (created_at);

create trigger enforce_date_format_trigger
    before insert or update
                         on meetings
                         for each row
                         execute procedure enforce_date_format();

create trigger validate_half_month_rule_trigger
    before insert or update
                         on meetings
                         for each row
                         execute procedure validate_half_month_rule();

create table commission_members
(
    id            uuid not null
        primary key,
    member_id     uuid not null
        references members,
    commission_id uuid not null
        references commissions,
    created_at    timestamp,
    updated_at    timestamp,
    unique (member_id, commission_id)
);

alter table commission_members
    owner to admin;

create index idx_commission_members_commission_id
    on commission_members (commission_id);

create index idx_commission_members_member_id
    on commission_members (member_id);

create unique index idx_commission_members_unique_pair
    on commission_members (member_id, commission_id);

create function validate_commission_chairman() returns trigger
    language plpgsql
as
$$
BEGIN
    IF NEW.chairman_id IS NOT NULL THEN
        IF (SELECT experience FROM members WHERE id = NEW.chairman_id) < 7 THEN
            RAISE EXCEPTION 'Chairman must be a member with experience > 7 years';
END IF;
END IF;
RETURN NEW;
END;
$$;

alter function validate_commission_chairman() owner to admin;

create function enforce_date_format() returns trigger
    language plpgsql
as
$$
BEGIN
    NEW.date := TO_DATE(TO_CHAR(NEW.date, 'DD.MM.YYYY'), 'DD.MM.YYYY');
RETURN NEW;
END;
$$;

alter function enforce_date_format() owner to admin;

create function validate_half_month_rule() returns trigger
    language plpgsql
as
$$
DECLARE
last_date DATE;
BEGIN
SELECT MAX(date) INTO last_date
FROM meetings
WHERE commission_id = NEW.commission_id AND date < NEW.date;

IF last_date IS NOT NULL AND NEW.date - last_date < 15 THEN
        RAISE EXCEPTION 'Meetings must be at least 15 days apart for the same commission';
END IF;

RETURN NEW;
END;
$$;

alter function validate_half_month_rule() owner to admin;

create function validate_unique_chairman_per_commission() returns trigger
    language plpgsql
as
$$
BEGIN
    IF NEW.chairman_id IS NOT NULL THEN
        IF (SELECT COUNT(*) FROM commissions WHERE chairman_id = NEW.chairman_id) > 1 THEN
            RAISE EXCEPTION 'A chairman can only lead one commission at a time';
END IF;
END IF;

RETURN NEW;
END;
$$;

alter function validate_unique_chairman_per_commission() owner to admin;


create function check_chairman_experience() returns trigger
    language plpgsql
as
$$
BEGIN
    IF NEW.is_chairman IS TRUE AND NEW.experience <= 7 THEN
        RAISE EXCEPTION 'Only members with more than 7 years of experience can be chairman';
END IF;

RETURN NEW;
END;
$$;

alter function check_chairman_experience() owner to admin;

create type commission_direction as enum ('Social', 'Ecological');

alter type commission_direction owner to admin;

