"use strict";

const monthNames = ["January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"];

const toDate = (iso) => {
    iso = autoIso(iso);

    if (iso)
        return new Date(iso);
    else
        return new Date();
}

const toIso = (date) => {
    date = toDate(date);
    return trimTimeZone(date.toISOString());
}

const toUtc = (iso) => {
    const date = toDate(iso);
    return trimTimeZone(date.toISOString());
}

const isIso = (iso) => {
    const regExp = new RegExp(`\\d{4}-?\\d{2}-?\\d{2}[T_]?[\\d]*:?[\\d]*:?[\\d]*\.?\\d*Z?$`);
    return regExp.test(iso);
};

const autoIso = (iso) => {
    if (!iso || typeof(iso) === "object")
        return iso;


    let dt = iso.toString();

    dt = dt.replace(/[-:T_]/g, '');

    let [date, y, m, d] = dt.match(/^(\d{4})(\d{2})(\d{2})/);
    if (m > 12)
        m = padValue(12);

    const maxM = new Date(y, m, 0).getDate();

    if (d > maxM)
        d = padValue(maxM);

    let hh = dt.substr(8, 2);
    let mm = dt.substr(10, 2);
    let ss = dt.substr(12, 2);

    if (!hh)
        hh = padValue(0);
    else if (hh > 23)
        hh = padValue(23);
    else
        hh = padValue(hh)

    if (!mm)
        mm = padValue(0);
    else if (mm > 59)
        mm = padValue(59);
    else
        mm = padValue(mm)

    if (!ss)
        ss = padValue(0);
    else if (ss > 59)
        ss = padValue(59);
    else
        ss = padValue(ss)

    const fdate = addTimeZone([
        [y, m, d].join('-'), [hh, mm, ss].join(':')
    ].join('T'));

    return fdate;
};

const shortMonth = (month) => {
    const m = Number.isInteger(month) ? monthNames[month] : month;
    return [m.charAt(0).toUpperCase(), m.substring(1, 3).toLowerCase()].join('');
};

const padValue = (str, len) => {
    str = str.toString();
    if (!len)
        len = 2;

    const pat = "0";
    const dif = len - str.length;
    const pad = len - dif > 0 ? pat.repeat(dif) : "";

    return pad + str;
};

const prettyDate = (iso) => {
    const date = toDate(iso);
    const d = date.getUTCDate();
    const m = shortMonth(date.getUTCMonth());
    const y = date.getUTCFullYear();
    const h = date.getUTCHours();
    const n = date.getUTCMinutes();
    const s = date.getUTCSeconds();

    const pdate = [
        [padValue(d), m, y].join("-"), [padValue(h), padValue(n), padValue(s)].join(":")
    ].join(' ');

    return pdate;
};

const ceilTime = (iso) => {
    const date = toDate(iso);
    const time = date.getMinutes();

    date.setMinutes(time + 1);
    date.setSeconds(0);

    return trimTimeZone(toIso(date));
}

const inRange = (date, ranges) => {
    let found = false;

    ranges.forEach(rng => {
        if (typeof(rng) === "object") {
            const arr = rng.sort();
            found = date >= rng[0] && date <= rng[rng.length - 1];
        } else {
            const arr = ranges.sort();
            found = date >= ranges[0] && date <= ranges[1];
        }

        if (found)
            return found;
    });

    return found;
};

const splitRange = (beg, end, mins) => {
    let rng = [];

    beg = toDate(beg);
    end = toDate(end);

    while (beg < end) {
        const diff = diffMinutes(beg, end);
        const len = diff < mins ? diff : mins;
        const mid = shiftMinutes(end, len * -1, true);

        rng.push([mid, end].map(date => toIso(date)));
        end = mid;
    };
    return rng;
};

const diffDates = (beg, end) => {
    beg = toDate(beg);
    end = toDate(end);

    return end.getTime() - beg.getTime();
};

const diffMinutes = (beg, end) => {
    return diffSeconds(beg, end) / 60;
};

const diffSeconds = (beg, end) => {
    return diffDates(beg, end) / 1000;
};

const shiftDays = (iso, shift, orig) => {
    const date = toDate(iso);
    date.setDate(date.getDate() + shift);
    if (orig)
        return date;
    return toIso(date);
};

const shiftHours = (iso, shift, orig) => {
    const date = toDate(iso);
    const time = date.getHours();
    date.setHours(time + shift);
    if (orig)
        return date;
    return toIso(date);
};

const shiftMinutes = (iso, shift, orig) => {
    const date = toDate(iso);
    const time = date.getMinutes();
    date.setMinutes(time + shift);
    if (orig)
        return date;

    return toIso(date);
};

const shiftSeconds = (iso, shift, orig) => {
    const date = toDate(iso);
    const time = date.getSeconds();
    if (orig)
        return date;
    return toIso(date.setSeconds(time + shift));
};

const trimTimeZone = (date) => {
    return date.substring(0, 19);
};

const addTimeZone = (date) => {
    return date.length === 19 ? date + ".000Z" : date;
};

module.exports = {
    isIso,
    toIso,
    toUtc,
    toDate,
    inRange,
    autoIso,
    ceilTime,
    shiftDays,
    diffDates,
    shiftHours,
    splitRange,
    prettyDate,
    addTimeZone,
    diffMinutes,
    diffSeconds,
    shiftMinutes,
    shiftSeconds,
    trimTimeZone
};