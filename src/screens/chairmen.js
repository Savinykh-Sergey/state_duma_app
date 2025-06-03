async function renderChairmen() {
    const table = document.getElementById('chairmenTable');
    table.innerHTML = '';
    try {
        const members = await invoke('fetch_chairmen_ui');
        if (!members || members.length === 0) {
            table.innerHTML = '<tr><td colspan="6">Нет данных</td></tr>';
            return;
        }
        members.forEach(member => {
            const birthday = member.birthday ? new Date(member.birthday * 1000).toISOString().split('T')[0] : '-';
            table.innerHTML += `
                <tr>
                    <td class="p-2 border-t">${member.full_name || '-'}</td>
                    <td class="p-2 border-t">+7${member.phone || ''}</td>
                    <td class="p-2 border-t">${birthday}</td>
                    <td class="p-2 border-t">${member.experience || '-'}</td>
                    <td class="p-2 border-t">
                        <button onclick="deleteChairman('${member.id.replace(/'/g, "\\'")}')" class="bg-red-500 text-white p-1 rounded">Удалить</button>
                    </td>
                </tr>
            `;
        });
    } catch (error) {
        console.error('Ошибка при загрузке членов:', error);
        table.innerHTML = '<tr><td colspan="6">Ошибка загрузки данных</td></tr>';
    }
}

async function deleteChairman(id) {
    if (confirm('Вы уверены, что хотите удалить этого члена думы?')) {
        try {
            await invoke('execute_delete_chairman_ui', {id});
            await renderChairmen();
        } catch (error) {
            console.error('Ошибка при удалении члена:', error);
            alert('Не удалось удалить члена: ' + error);
        }
    }
}

function closeModal(modalId) {
    document.getElementById(modalId).classList.add('hidden');
}