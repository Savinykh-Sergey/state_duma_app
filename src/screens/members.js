let currentMemberId = null;

async function renderMembers() {
    const table = document.getElementById('membersTable');
    table.innerHTML = '';
    try {
        const members = await invoke('fetch_members_ui');
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
                        <button onclick="editMember('${member.id.replace(/'/g, "\\'")}')" class="bg-yellow-500 text-white p-1 rounded">Редактировать</button>
                        <button onclick="deleteMember('${member.id.replace(/'/g, "\\'")}')" class="bg-red-500 text-white p-1 rounded">Удалить</button>
                    </td>
                </tr>
            `;
        });
    } catch (error) {
        console.error('Ошибка при загрузке членов:', error);
        table.innerHTML = '<tr><td colspan="6">Ошибка загрузки данных</td></tr>';
    }
}

async function populateMemberCommissionSelect() {
    try {
        const commissions = await invoke('fetch_commissions_ui');
        const select = document.getElementById('memberCommission');
        select.innerHTML = '<option value="">Выберите комиссию</option>';
        commissions.forEach(commission => {
            const direction = commission.direction === 'Social' ? 'Социальная' : 'Экологическая';
            select.innerHTML += `<option value="${commission.id}">${direction}</option>`;
        });
    } catch (error) {
        console.error('Ошибка при загрузке комиссий для члена:', error);
    }
}

async function showAddMemberForm() {
    document.getElementById('memberFormTitle').textContent = 'Добавить члена думы';
    document.getElementById('memberName').value = '';
    document.getElementById('memberPhone').value = '';
    document.getElementById('memberBirthDate').value = '';
    document.getElementById('memberExperience').value = '';
    document.getElementById('memberIsChairman').checked = false;
    document.getElementById('memberCommission').selectedIndex = 0;
    currentMemberId = null;
    await populateMemberCommissionSelect();
    closeModal('memberModal');
    document.getElementById('memberModal').classList.remove('hidden');
}

async function editMember(id) {
    try {
        const member = await invoke('fetch_member_ui', { query: id });
        if (member) {
            document.getElementById('memberFormTitle').textContent = 'Редактировать члена думы';
            document.getElementById('memberName').value = member.full_name || '';
            document.getElementById('memberPhone').value = `+7${member.phone || ''}`
            document.getElementById('memberBirthDate').value = member.birthday ? new Date(member.birthday * 1000).toISOString().split('T')[0] : '';
            document.getElementById('memberExperience').value = member.experience || '';
            document.getElementById('memberIsChairman').checked = member.is_chairman || false;
            currentMemberId = id;
            await populateMemberCommissionSelect();
            if (member.commission_id) {
                document.getElementById('memberCommission').value = member.commission_id;
            } else {
                document.getElementById('memberCommission').selectedIndex = 0;
            }
            document.getElementById('memberModal').classList.remove('hidden');
        } else {
            console.error('Член с id', id, 'не найден');
        }
    } catch (error) {
        console.error('Ошибка при редактировании члена:', error);
    }
}

async function deleteMember(id) {
    if (confirm('Вы уверены, что хотите удалить этого члена думы?')) {
        try {
            await invoke('execute_delete_member_ui', { id });
            await renderMembers();
        } catch (error) {
            console.error('Ошибка при удалении члена:', error);
            alert('Не удалось удалить члена: ' + error);
        }
    }
}

async function saveMember() {
    const fullName = document.getElementById('memberName').value || null;
    const phoneInput = document.getElementById('memberPhone').value || '';
    const birthday = document.getElementById('memberBirthDate').value || '';
    const experienceInput = document.getElementById('memberExperience').value;
    const experience = experienceInput ? parseInt(experienceInput) : null;
    const isChairman = document.getElementById('memberIsChairman').checked;
    const commissionId = document.getElementById('memberCommission').value;

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
    if (!experienceInput) {
        alert('Поле "Опыт" не может быть пустым. Пожалуйста, укажите опыт.');
        return;
    }

    const normalizedPhone = normalizePhoneNumber(phoneInput, true);
    if (!normalizedPhone) {
        alert('Неверный формат телефона. Убедитесь, что номер содержит 11 цифр и начинается с 7, или 12-16 цифр для мобильного.');
        return;
    }


    try {
        if (!currentMemberId) {
            await invoke('execute_create_member_ui', {
                fullName,
                birthday,
                experience,
                phone: normalizedPhone,
                isChairman
            });
        } else {
            let member = await invoke('fetch_member_ui', { query: fullName });

            await invoke('execute_edit_member_ui', {
                id: member.id,
                fullName,
                birthday,
                experience,
                phone: normalizedPhone,
                isChairman
            });
        }

        let member = await invoke('fetch_member_ui', { query: fullName });

        if (commissionId) {
            let commissionMember = null;
            try {
                commissionMember = await invoke('fetch_commission_member_ui', { memberId: member.id });
            } catch (innerError) {
                console.error('Ошибка при получении данных связи для члена:', innerError);
            }

            console.log('commissionMember', commissionMember);

            if (commissionMember) {
                if (commissionMember.commission_id === commissionId) {
                    console.log('Член уже находится в выбранной комиссии.');
                } else {
                    await invoke('execute_edit_commission_members_ui', {
                        memberId: member.id,
                        commissionId
                    });
                }
            } else {
                await invoke('execute_create_commission_members_ui', {
                    memberId: member.id,
                    commissionId
                });
            }
        }

        await renderMembers();
        closeModal('memberModal');
    } catch (error) {
        console.error('Ошибка при сохранении члена:', error);
        alert('Не удалось сохранить члена: ' + error);
    }
}

function closeModal(modalId) {
    document.getElementById(modalId).classList.add('hidden');
}