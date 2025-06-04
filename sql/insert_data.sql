INSERT INTO members (id, full_name, birthday, experience, phone, is_chairman, created_at, updated_at)
VALUES
    (uuid_generate_v4(), 'Винский Борис Петрович', '1992-02-23', 6, '9787463546', False, NOW(), NOW()),
    (uuid_generate_v4(), 'Аталиков Павел Соснович', '1975-03-24', 3, '9764976578', False, NOW(), NOW()),
    (uuid_generate_v4(), 'Рыжий Степан Аркадьевич', '1899-07-12', 10, '9863826754', True, NOW(), NOW()),
    (uuid_generate_v4(), 'Вислюков Анабел Бенедиктович', '1990-10-19', 2, '9678265493', False, NOW(), NOW()),
    (uuid_generate_v4(), 'Ахметкин Петр Иванович', '1991-12-23', 9, '9797464598', True,NOW(), NOW());

INSERT INTO organizers (id, full_name, phone, birthday, created_at, updated_at)
VALUES
    (uuid_generate_v4(), 'Попов Сергей Степанович', '9797453542', '1993-12-02', NOW(), NOW()),
    (uuid_generate_v4(), 'Прокопьев Алексей Анатольевич', '9797464598', '1991-12-23', NOW(), NOW());

INSERT INTO commissions (id, direction, organizer_id, chairman_id, created_at, updated_at)
VALUES
    (uuid_generate_v4(), 'Social', (SELECT id FROM organizers WHERE full_name='Попов Сергей Степанович'), (SELECT id FROM members WHERE full_name='Рыжий Степан Аркадьевич'), NOW(), NOW()),
    (uuid_generate_v4(), 'Ecological', (SELECT id FROM organizers WHERE full_name='Прокопьев Алексей Анатольевич'), (SELECT id FROM members WHERE full_name='Ахметкин Петр Иванович'), NOW(), NOW());

INSERT INTO meetings (id, date, duration, commission_id, created_at, updated_at)
VALUES
    (uuid_generate_v4(), '2016-02-15', 2, (SELECT id FROM commissions WHERE direction='Social'), NOW(), NOW()),
    (uuid_generate_v4(), '2016-03-16', 2, (SELECT id FROM commissions WHERE direction='Social'), NOW(), NOW()),
    (uuid_generate_v4(), '2016-04-21', 2, (SELECT id FROM commissions WHERE direction='Ecological'), NOW(), NOW());

INSERT INTO commission_members (id, member_id, commission_id, created_at, updated_at)
VALUES
    (uuid_generate_v4(), (SELECT id FROM members WHERE full_name='Винский Борис Петрович'), (SELECT id FROM commissions WHERE direction='Social'), NOW(), NOW()),
    (uuid_generate_v4(), (SELECT id FROM members WHERE full_name='Аталиков Павел Соснович'), (SELECT id FROM commissions WHERE direction='Social'), NOW(), NOW()),
    (uuid_generate_v4(), (SELECT id FROM members WHERE full_name='Рыжий Степан Аркадьевич'), (SELECT id FROM commissions WHERE direction='Social'), NOW(), NOW()),
    (uuid_generate_v4(), (SELECT id FROM members WHERE full_name='Вислюков Анабел Бенедиктович'), (SELECT id FROM commissions WHERE direction='Ecological'), NOW(), NOW());
