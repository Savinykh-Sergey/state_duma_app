async function generateMeetingsReport() {
    const yearInput = document.getElementById('reportMeetingsYear');
    const year = parseInt(yearInput.value);
    const table = document.getElementById('reportMeetingsTable');
    table.innerHTML = '';

    try {
        const reportData = await invoke('fetch_meetings_by_year_ui', {year});

        if (!reportData || reportData.length === 0) {
            table.innerHTML = '<tr><td colspan="2">Нет данных</td></tr>';
            return;
        }

        let month_arr = ['Январь', 'Февраль', 'Март', 'Апрель', 'Май', 'Июнь', 'Июль', 'Август', 'Сентябрь', 'Октябрь', 'Ноябрь', 'Декабрь'];

        reportData.forEach(item => {
            table.innerHTML += `
                <tr>
                    <td class="p-2 border-t">${month_arr[item.month]}</td>
                    <td class="p-2 border-t">${item.count}</td>
                </tr>
            `;
        });
    } catch (error) {
        console.error('Ошибка при загрузке отчёта заседания по месяцам:', error);
        table.innerHTML = '<tr><td colspan="2">Ошибка загрузки данных</td></tr>';
    }
}
