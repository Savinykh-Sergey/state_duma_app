function normalizePhoneNumber(phone, onlyMobile = false) {
    console.log('Проверка телефона:', phone);
    if (!phone || phone.trim() === '') {
        console.log('Телефон пустой');
        return null;
    }

    const digits = phone.replace(/\D/g, '');
    console.log('Только цифры:', digits);

    if ((digits.startsWith('7') || digits.startsWith('8')) && digits.length === 11) {
        const normalized = digits.slice(1);
        console.log('Нормализованный номер (11 цифр):', normalized);
        return normalized;
    } else if (digits.length === 10 && !onlyMobile) {
        const normalized = digits;
        console.log('Нормализованный номер (10 цифр):', normalized);
        return normalized;
    } else if (digits.length > 11 && digits.length <= 16 && onlyMobile) {
        console.log('Длинный номер (мобильный):', digits);
        return digits;
    }

    console.log('Номер не соответствует условиям');
    return null;
}