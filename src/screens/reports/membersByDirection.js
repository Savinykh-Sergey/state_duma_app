async function generateCommissionReport() {
    const directionSelect = document.getElementById('reportCommissionDirection');
    const direction = directionSelect.value;

    const table = document.getElementById('reportCommissionTable');
    table.innerHTML = '';

    try {
        const reportData = await invoke('fetch_members_by_direction_report_ui', { direction });

        if (!reportData || reportData.length === 0) {
            table.innerHTML = '<tr><td colspan="2">Нет данных</td></tr>';
            return;
        }

        reportData.forEach(member => {
            const role = member.is_chairman ? 'Председатель' : 'Член комиссии';
            table.innerHTML += `
                <tr>
                    <td class="p-2 border-t">${member.full_name}</td>
                    <td class="p-2 border-t">${role}</td>
                </tr>
            `;
        });
    } catch (error) {
        console.error('Ошибка при генерации отчета по комиссии:', error);
        table.innerHTML = '<tr><td colspan="2">Ошибка загрузки данных</td></tr>';
    }
}