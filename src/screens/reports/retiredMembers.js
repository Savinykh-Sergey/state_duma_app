async function renderReportRetired() {
    const table = document.getElementById('reportPensionersTable');
    table.innerHTML = '';
    try {
        const experiencedMembers = await invoke('fetch_report_retired_member_ui');
        if (!experiencedMembers || experiencedMembers.length === 0) {
            table.innerHTML = '<tr><td colspan="2">Нет данных</td></tr>';
            return;
        }

        experiencedMembers.forEach(member => {
            table.innerHTML += `
                <tr>
                    <td class="p-2 border-t">${member.full_name}</td>
                    <td class="p-2 border-t">${member.birthday}</tr>
                </tr>
            `;
        });
    } catch (error) {
        console.error('Ошибка при загрузке отчёта по стажу членов:', error);
        table.innerHTML = '<tr><td colspan="2">Ошибка загрузки данных</td></tr>';
    }
}

// function closeModal(modalId) {
//     document.getElementById(modalId).classList.add('hidden');
// }