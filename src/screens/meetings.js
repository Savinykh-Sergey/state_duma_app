let currentMeetingId = null;
async function renderMeetings() {
    const table = document.getElementById('meetingsTable');
    table.innerHTML = '';
    try {
        const meetings = await invoke('fetch_meetings_ui');
        const commissions = await invoke('fetch_commissions_ui');

        meetings.forEach(meeting => {
            const commission = commissions.find(c => c.id === meeting.commission_id)?.direction || 'Неизвестно';
            table.innerHTML += `
                <tr>
                    <td class="p-2 border-t">${meeting.date}</td>
                    <td class="p-2 border-t">${meeting.duration} мин</td>
                    <td class="p-2 border-t">${commission === 'Social' ? 'Социальное' : 'Экологическое'}</td>
                    <td class="p-2 border-t">
                        <button onclick="deleteMeeting('${meeting.id}')" class="bg-red-500 text-white p-1 rounded">Удалить</button>
                    </td>
                </tr>
            `;
        });
    } catch (error) {
        console.error('Ошибка при загрузке заседаний:', error);
        table.innerHTML = '<tr><td colspan="4">Ошибка загрузки данных</td></tr>';
    }
}

async function populateMeetingCommissionSelect() {
    try {
        const commissions = await invoke('fetch_commissions_ui');
        const chairmen = await invoke('fetch_chairmen_ui');

        const select = document.getElementById('meetingCommission');
        select.innerHTML = '<option value="">Выберите комиссию</option>';

        commissions.forEach(commission => {
            const directionLabel = commission.direction === 'Social' ? 'Социальное' : 'Экологическое';
            const chairman = chairmen.find(chair => chair.id === commission.chairman_id);
            const chairmanName = chairman ? chairman.full_name : 'Не указан';
            select.innerHTML += `<option value="${commission.id}">${directionLabel} – Председатель: ${chairmanName}</option>`;
        });
    } catch (error) {
        console.error('Ошибка при загрузке данных комиссий для заседания:', error);
    }
}

async function showAddMeetingForm() {
    document.getElementById('meetingFormTitle').textContent = 'Добавить заседание';
    document.getElementById('meetingDate').value = '';
    document.getElementById('meetingDuration').value = '';
    await populateMeetingCommissionSelect();
    currentMeetingId = null;
    document.getElementById('meetingModal').classList.remove('hidden');
}

async function deleteMeeting(id) {
    if (confirm('Вы уверены, что хотите удалить это заседание?')) {
        try {
            await invoke('execute_delete_meeting_ui', { id });
            await renderMeetings();
        }
        catch (error) {
            console.error('Ошибка при удалении заседания:', error);
            alert('Не удалось удалить заседание: ' + error);
        }
    }
}

async function saveMeeting() {
    const id = currentMeetingId || '';
    const date = document.getElementById('meetingDate').value;
    const duration = parseInt(document.getElementById('meetingDuration').value, 10);
    const commissionId = document.getElementById('meetingCommission').value || null;

    if (!date) {
        alert('Пожалуйста, выберите дату заседания.');
        return;
    }
    if (!duration) {
        alert('Пожалуйста, укажите продолжительность.');
        return;
    }
    if (!commissionId) {
        alert('Пожалуйста, выберите комиссию.');
        return;
    }
    try {
        if (!currentMeetingId) {
            await invoke('execute_create_meeting_ui', {
                date,
                duration,
                commissionId
            });
        } else {
            await invoke('execute_edit_meeting_ui', {
                id,
                date,
                duration,
                commissionId
            });
        }
        await renderMeetings();
        closeModal('meetingModal');
    }
    catch (error) {
        console.error('Ошибка при сохранении заседания:', error);
        alert('Не удалось сохранить заседание: ' + error);
    }
}

function closeModal(modalId) {
    document.getElementById(modalId).classList.add('hidden');
}
