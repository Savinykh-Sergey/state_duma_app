async function renderReportExperience() {
    const table = document.getElementById('reportExperienceTable');
    table.innerHTML = '';
    try {
        const experiencedMembers = await invoke('fetch_report_experienced_member_ui');
        if (!experiencedMembers || experiencedMembers.length === 0) {
            table.innerHTML = '<tr><td colspan="2">Нет данных</td></tr>';
            return;
        }

        experiencedMembers.forEach(member => {
            table.innerHTML += `
                <tr>
                    <td class="p-2 border-t">${member.full_name}</td>
                    <td class="p-2 border-t">${member.experience}</td>
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