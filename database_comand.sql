create database QuotaManageSystem;

create table quota_control_field(
    id varchar(255) PRIMARY KEY NOT NULL,
    quota_control_field text NOT NULL,
    explain_info jsonb NOT NULL,
    state varchar(255) NOT NULL,
    create_time timestamp NOT NULL,
    update_time timestamp NOT NULL);