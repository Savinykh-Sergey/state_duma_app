let currentOrganizerId = null;

async function renderOrganizers() {
    const table = document.getElementById('organizersTable');
    table.innerHTML = '';
    try {
        const organizers = await invoke('fetch_organizers_ui');
        if (!organizers || organizers.length === 0) {
            table.innerHTML = '<tr><td colspan="6">Нет данных</td></tr>';
            return;
        }
        organizers.forEach(organizer => {
            const birthday = organizer.birthday ? new Date(organizer.birthday * 1000).toISOString().split('T')[0] : '-'
            table.innerHTML += `
                <tr>
                    <td class="p-2 border-t">${organizer.full_name || '-'}</td>
                    <td class="p-2 border-t">+7${organizer.phone || ''}</td>
                    <td class="p-2 border-t">${birthday || '-'}</td>
                    <td class="p-2 border-t">
                        <button onclick="editOrganizer('${organizer.id.replace(/'/g, "\\'")}')" class="bg-yellow-500 text-white p-1 rounded">Редактировать</button>
                        <button onclick="deleteOrganizer('${organizer.id.replace(/'/g, "\\'")}')" class="bg-red-500 text-white p-1 rounded">Удалить</button>
                    </td>
                </tr>
            `;
        });
    } catch (error) {
        console.error('Ошибка при загрузке организаторов:', error);
        table.innerHTML = '<tr><td colspan="6">Ошибка загрузки данных</td></tr>';
    }
}

async function showAddOrganizerForm() {
    document.getElementById('organizerFormTitle').textContent = 'Добавить организатора';
    document.getElementById('organizerName').value = '';
    document.getElementById('organizerPhone').value = '';
    document.getElementById('organizerBirthDate').value = '';
    currentOrganizerId = null;
    await renderOrganizers();
    closeModal('organizerModal');
    document.getElementById('organizerModal').classList.remove('hidden');
}

async function editOrganizer(id) {
    try {
        const organizer = await invoke('fetch_organizer_ui', { id });
        if (organizer) {
            document.getElementById('organizerFormTitle').textContent = 'Редактировать организатора';
            document.getElementById('organizerName').value = organizer.full_name;
            document.getElementById('organizerPhone').value = `+7${organizer.phone || ''}`
            document.getElementById('organizerBirthDate').value = organizer.birthday ? new Date(organizer.birthday * 1000).toISOString().split('T')[0] : '';
            currentOrganizerId = id;
            document.getElementById('organizerModal').classList.remove('hidden');
        } else {
            console.error('Организатор с id', id, 'не найден');
        }
    } catch (error) {
        console.error('Ошибка при редактировании организатор:', error);
    }
}

async function deleteOrganizer(id) {
    if (confirm('Вы уверены, что хотите удалить этого организатора?')) {
        try {
            await invoke('execute_delete_organizer_ui', {id});
            await renderOrganizers();
        } catch (error) {
            console.error('Ошибка при удалении организатора:', error);
            alert('Не удалось удалить организатора: ' + error);
        }
    }
}

async function saveOrganizer() {
    const id = currentOrganizerId || '';
    const fullName = document.getElementById('organizerName').value || '';
    const phoneInput = document.getElementById('organizerPhone').value || '';
    const birthday = document.getElementById('organizerBirthDate').value || '';

    if (!fullName) {
        alert('Поле "Имя" не может быть пустым. Пожалуйста, заполните его.');
        return;
    }
    if (!phoneInput) {
        alert('Поле "Телефон" не может быть пустым. Пожалуйста, заполните его.');
        return;
    }
    if (!birthday) {
        alert('Поле "Дата рождения" не может быть пустым. Пожалуйста, выберите дату.');
        return;
    }

    const normalizedPhone = normalizePhoneNumber(phoneInput, true);
    console.log('Результат нормализации телефона:', normalizedPhone);
    if (!normalizedPhone) {
        console.log('Телефон некорректный');
        alert('Неверный формат телефона. Убедитесь, что номер содержит 11 цифр и начинается с 7 или 8.');
        return;
    }

    console.log('Отправляемые данные:', {id, fullName, birthday, phone: normalizedPhone});

    try {
        if (!currentOrganizerId) {
            await invoke('execute_create_organizer_ui', {
                id,
                fullName,
                birthday,
                phone: normalizedPhone,
            });
        } else {
            await invoke('execute_edit_organizer_ui', {
                id,
                fullName,
                birthday,
                phone: normalizedPhone,
            });
        }
        await renderOrganizers();
        closeModal('organizerModal');
    } catch (error) {
        console.error('Ошибка при сохранении организатора:', error);
        alert('Не удалось сохранить организатора: ' + error);
    }
}

function closeModal(modalId) {
    document.getElementById(modalId).classList.add('hidden');
}