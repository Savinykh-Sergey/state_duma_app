let currentCommissionId = null;

async function renderCommissions() {
    const table = document.getElementById('commissionsTable');
    table.innerHTML = '';
    try {
        const commissions = await invoke('fetch_commissions_ui');
        const chairmen = await invoke('fetch_chairmen_ui');
        const organizers = await invoke('fetch_organizers_ui');

        if (!commissions || commissions.length === 0) {
            table.innerHTML = '<tr><td colspan="4">Нет данных</td></tr>';
            return;
        }

        commissions.forEach(commission => {
            console.log(commission);
            const chairman = chairmen.find(c => c.id === commission.chairman_id) || { full_name: 'Не указан' };
            const organizer = organizers.find(o => o.id === commission.organizer_id) || { full_name: 'Не указан' };
            table.innerHTML += `
                <tr>
                    <td class="p-2 border-t">${commission.direction === 'Social' ? 'Социальное' : 'Экологическое'}</td>
                    <td class="p-2 border-t">${chairman.full_name}</td>
                    <td class="p-2 border-t">${organizer.full_name}</td>
                    <td class="p-2 border-t">
                        <button onclick="editCommission('${commission.id.replace(/'/g, "\\'")}')" class="bg-yellow-500 text-white p-1 rounded">Редактировать</button>
                        <button onclick="deleteCommission('${commission.id.replace(/'/g, "\\'")}')" class="bg-red-500 text-white p-1 rounded">Удалить</button>
                    </td>
                </tr>
            `;
        });
    } catch (error) {
        console.error('Ошибка при загрузке комиссий:', error);
        table.innerHTML = '<tr><td colspan="4">Ошибка загрузки данных</td></tr>';
    }
}

async function showAddCommissionForm() {
    document.getElementById('commissionFormTitle').textContent = 'Добавить комиссию';
    document.getElementById('commissionDirectionInput').value = 'Social';

    await populateCommissionSelects();

    currentCommissionId = null;
    await renderCommissions();
    closeModal('commissionModal');
    document.getElementById('commissionModal').classList.remove('hidden');
}

async function populateCommissionSelects(commission = null) {
    try {
        const chairmen = await invoke('fetch_chairmen_ui');
        const organizers = await invoke('fetch_organizers_ui');

        const chairmanSelect = document.getElementById('commissionChairman');
        const organizerSelect = document.getElementById('commissionOrganizer');

        chairmanSelect.innerHTML = '<option value="">Выберите председателя</option>';
        organizerSelect.innerHTML = '<option value="">Выберите организатора</option>';

        chairmen.forEach(chairman => {
            chairmanSelect.innerHTML += `<option value="${chairman.id}" ${commission && commission.chairman_id === chairman.id ? 'selected' : ''}>${chairman.full_name}</option>`;
        });

        organizers.forEach(organizer => {
            organizerSelect.innerHTML += `<option value="${organizer.id}" ${commission && commission.organizer_id === organizer.id ? 'selected' : ''}>${organizer.full_name}</option>`;
        });
    } catch (error) {
        console.error('Ошибка при загрузке данных для выпадающих списков:', error);
    }
}

async function editCommission(id) {
    try {
        const commission = await invoke('fetch_commission_ui', { id });
        if (commission) {
            document.getElementById('commissionFormTitle').textContent = 'Редактировать комиссию';
            document.getElementById('commissionDirectionInput').value = commission.direction;

            await populateCommissionSelects(commission);

            currentCommissionId = id;
            document.getElementById('commissionModal').classList.remove('hidden');
        } else {
            console.error('Комиссия с id', id, 'не найдена');
        }
    } catch (error) {
        console.error('Ошибка при редактировании комиссии:', error);
    }
}

async function deleteCommission(id) {
    if (confirm('Вы уверены, что хотите удалить эту комиссию?')) {
        try {
            await invoke('execute_delete_commission_ui', { id });
            await renderCommissions();
        } catch (error) {
            console.error('Ошибка при удалении комиссии:', error);
            alert('Не удалось удалить комиссию: ' + error);
        }
    }
}

async function saveCommission() {
    const id = currentCommissionId || '';
    const direction = document.getElementById('commissionDirectionInput').value;
    const chairmanId = document.getElementById('commissionChairman').value || null;
    const organizerId = document.getElementById('commissionOrganizer').value || null;

    if (!chairmanId) {
        alert('Пожалуйста, выберите председателя.');
        return;
    }
    if (!organizerId) {
        alert('Пожалуйста, выберите организатора.');
        return;
    }

    try {
        if (!currentCommissionId) {
            await invoke('execute_create_commission_ui', {
                id,
                direction: direction === 'social' ? 'Social' : 'Ecological',
                chairmanId,
                organizerId
            });
        } else {
            await invoke('execute_edit_commission_ui', {
                id,
                direction: direction === 'social' ? 'Social' : 'Ecological',
                chairmanId,
                organizerId
            });
        }
        await renderCommissions();
        closeModal('commissionModal');
    } catch (error) {
        console.error('Ошибка при сохранении комиссии:', error);
        alert('Не удалось сохранить комиссию: ' + error);
    }
}
function closeModal(modalId) {
    document.getElementById(modalId).classList.add('hidden');
}