function showSubMenu(menuId) {
    document.getElementById('mainMenu').classList.add('hidden');
    document.getElementById('membersSubMenu').classList.add('hidden');
    document.getElementById('commissionsSubMenu').classList.add('hidden');
    document.getElementById('reportsSubMenu').classList.add('hidden');
    document.getElementById(menuId + 'SubMenu').classList.remove('hidden');
}

function backToMainMenu() {
    document.getElementById('mainMenu').classList.remove('hidden');
    document.getElementById('membersSubMenu').classList.add('hidden');
    document.getElementById('commissionsSubMenu').classList.add('hidden');
    document.getElementById('reportsSubMenu').classList.add('hidden');
    document.querySelectorAll('.section').forEach(section => {
        section.classList.add('hidden');
    });
}

function showSection(sectionId, beforeRenderCallback) {
    document.querySelectorAll('.section').forEach(section => {
        section.classList.add('hidden');
    });

    if (beforeRenderCallback) {
        beforeRenderCallback();
    }

    document.getElementById(sectionId).classList.remove('hidden');
}

function closeModal(modalId) {
    document.getElementById(modalId).classList.add('hidden');
}
