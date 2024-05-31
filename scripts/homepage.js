var dates = [];
var dateRange = {
    "start": 0, // number of days since 1/1/1970
    "length": 7, // select the 7 days after this
    "offset": 0,
    "current": 0
}
// year, month, day offset
// var dateOffset = [0, 0, 0];

console.log(username);
console.log(password);

var container = document.querySelector("#main #content #field");
var library = [];

var taskHeight = window.getComputedStyle(container).getPropertyValue('--task-height');
var taskWidth = window.getComputedStyle(container).getPropertyValue('--task-width');

var creationContainer = document.querySelector("#task-creation");

var timePointerText = document.querySelector("#time-pointer-text");
var timePointer = document.querySelector("#content #time-pointer");
var verticalHeaderOffset = 0; // height of vertical headers

// #region colour themees
var colour_themes = [
    "crimson",
    "#ec5d2a",
    "forestgreen",
    "dodgerblue",
    "blueviolet"
    // "coral",
    // "lightsalmon",
    // "orange",
    // "sandybrown",
    // "tomato",
    // "dodgerblue",
    // "blueviolet",
    // "teal",
    // // "navy",
    // "chocolate",
    // // "darkviolet",
    // // "hotpink",
    // "lightslategrey",
    // // "orangered",
    // // "tomato"
    // "forestgreen"
];
// #endregion

document.querySelectorAll("#main table tbody td div h4")[clampValue(Math.floor(fetchCurrentTime()) + 2, 0, 23)].scrollIntoView({
    'behavior':'auto',
    'block':'center',
    'inline':'center'
});
document.querySelectorAll("#main #header-container .header")[clampValue(fetchDayIndex(), 0, 6)].scrollIntoView({
    'behavior':'auto',
    'block':'center',
    'inline':'center'
});

// #region utils
function login_info() {
    return JSON.stringify({
        "username":username,
        "password":password
    });
}

function getEpochDate(gmt=true, d=null) {
    // days since 1 jan 1970
    let now = d ? d : new Date();

    // let t = Date.parse(`${now.getUTCMonth() + 1}/${now.getUTCDate()}/${now.getUTCFullYear()} ${now.getUTCHours()}:${now.getUTCMinutes()}:${now.getUTCSeconds()} GMT`) / 1000;
    return Math.floor((now.getTime() - (gmt ? 0 : now.getTimezoneOffset() * 60_000)) / 86400_000);
}

function roundToWeekStart(i) {
    // takes days since 1/1/1970
    // returns the date of the monday on that week

    // 1/1/1970 is Thursday; offsetting to monday
    return (Math.floor((i + 3) / 7) * 7) - 3;
}

function roundToDayStart(gmt=true, d=null) {
    // 86399 -> 86399
    // 86400 -> 86400
    // 86401 -> 1
    let now = d ? d : new Date();

    return Math.floor(((now.getTime() / 1000) - (gmt ? 0 : now.getTimezoneOffset() * 60)) % 86400);
}

function cleanTimeInput(e, minute=false) {
    if (e.value.length > 2) {
        e.value = e.value.slice(0, 2);
    }
    let i = parseInt(e.value);
    if (i > (minute ? 59 : 23)) {
        e.value = (minute ? 59 : 23);
    }

    if (i < 0) {
        e.value = 0;
    }

    if (minute) {
        e.dataset.time_previous_minute = e.value;
    } else {
        e.dataset.time_previous_hour = e.value;
    }
}
// #endregion

function populateCalendar() {
    container.innerHTML = '';
    library.forEach((e) => {
        if (e["type"] == "container") {
            let batch = '';
            e['children'].forEach((i, index) => {
                batch += addItem(i, index + 1);
            })
            container.innerHTML += `<div class="item-container" style="top:calc(${taskHeight} * ${e['position'][1]}); left:calc(${taskWidth} * ${e['position'][0]}); height:calc(calc(${taskHeight} * ${e['height']}) - 4ch);">
    <h1>
        ${e['children'].length}
    </h1>
    <img src='/assets/right_chevron.png'>
    <div id="user-click" onclick="toggleItemContainer(this)">
    </div>
    <div id="children">
        ${batch}
    </div>
</div>`;
        } else {
            container.innerHTML += addItem(e);
        }
    })
}

function addItem(e, i=null) {
    let pos = convertEpochToUnitPosition(e);
    if (pos === null) { return ""; }
    // console.log(pos);
//     return `<div class="item" style="top:calc(${pos[1]} * ${taskHeight}); height:calc(calc(${pos[2]} * ${taskHeight}) - 4ch); left:${(i === null ? `calc(var(--task-width) * ${pos[0]})` : `calc(calc(var(--task-width) * ${i}) - 1ch)`)}; background:${colour_themes[e['colour']]};">
//     <h3 id="title">${e['title']}</h3>
// </div>`


    return `<div class="item" style="top:calc(${pos[1]} * ${taskHeight}); height:calc(calc(${pos[2]} * ${taskHeight}) - 4ch); left:${(i === null ? `calc(var(--task-width) * ${pos[0]})` : `calc(calc(var(--task-width) * ${i}) - 1ch)`)}; background:${colour_themes[e['colour']]};" ${e['species'] == 'Event' ? '' : `dataset-completed="${e['species']['Task']}"`}>
    <div>
        <img id="completion" src="/assets/uncompleted.png">
        <h3 id="title">${e['title']}</h3>
    </div>
    <h4 id="description">${e['description']}</h4>
    <h5 id="time"></h5>
</div>`

    // <h4 id="time">${e['time'][0]}-${e['time'][1]}</h4>
}

function fetchLibrary() {
    // return;
    sendPostRequest(`${BACKEND_ADDRESS}/fetch_library/0/1813657600`, login_info(), (r) => {
        let response = parseResponse(r);
        library = JSON.parse(response);

        console.log(response);

        gatherChildren();

        populateCalendar();

        // calculate verticalHeaderOffset
        verticalHeaderOffset = document.querySelector("#header-container").clientHeight + 'px';
    })
}

function gatherChildren() {
    let hashmap = {};

    library.forEach((e) => {
        let start = fetchTaskStart(e);

        if (start in hashmap) {
            hashmap[start].push(e);
        } else {
            hashmap[start] = [e];
        }
    })

    library = [];
    for (k in hashmap) {
        let v = hashmap[k];

        if (v.length > 1) {
            let tallest = 0;
            let pos = 0;
            v.forEach((e) => {
                pos = convertEpochToUnitPosition(e);

                if (pos[2] > tallest) {
                    tallest = pos[2];
                }
            })

            library.push({
                "type":"container",
                "children":v,
                "position":pos,
                "height":tallest,
            });
        } else {
            library.push(v[0]);
        }
    }
}

fetchLibrary();

// #region date related
function fetchDateRange() {
    // var start = getEpochDate();
    dates = [];
    dateRange['current'] = fetchDayIndex();
    dateRange['start'] = roundToWeekStart(getEpochDate(false) + dateRange['offset']) * 86400;
    for(i = 0; i < dateRange['length']; i++) {
        dates.push(dateRange['start'] + (i * 86400));
    }

    let headers = document.querySelectorAll("#main #header-container .header h4:last-of-type");
    dates.forEach((e, index) => {
        headers[index].innerHTML = new Date(e * 1000).getDate();
        headers[index].parentElement.parentElement.ariaLabel = index == dateRange['current'] ? 'highlighted' : '';
    })
}

fetchDateRange();

function fetchDayIndex(d=null) {
    // current day of the week
    // monday = 0
    // tuesday = 1
    // using own utils (for consistency purposes)
    return getEpochDate(false, d) - roundToWeekStart(getEpochDate(false, d));
}

function convertEpochToUnitPosition(e) {
    if (e['occurance_species'] != 'Once') { return null; }

    // takes epoch unix and changes position data
    // [x, y, h]
    let height = 1;
    let start = fetchTaskStart(e);
    if (["AllDay", "DayRange"].includes([Object.keys(e['time_species'])[0]])) {
        // ignore if AllDay or DayRange
        return null;
    }
    if (Object.keys(e['time_species'])[0] == "Range") {
        height = (e['time_species']['Range'][1] - e['time_species']['Range'][0]) / 3600;
    }
    return [
        fetchDayIndex(new Date(start * 1000)),
        Math.floor(start % 86400) / 3600,
        height
    ];
}

function fetchTaskStart(e) {
    let r = e['time_species'][Object.keys(e['time_species'])[0]];
    if (Array.isArray(r)) {
        return r[0];
    }
    return r;
}

function fetchTaskEnd(e) {
    let r = e['time_species'][Object.keys(e['time_species'])[0]];
    if (Array.isArray(r)) {
        return r[1];
    }
    return null;
}

function fetchCurrentTime(date=null) {
    // 0:00 -> 0
    // 1:00 -> 1
    // 1:30 -> 1.5
    // used for task top values

    let d = date ? date : new Date();
    return d.getHours() + (d.getMinutes() / 60);
}

function setTimePointer() {
    let d = new Date();
    let t = `calc(${fetchCurrentTime()} * var(--task-height))`;
    timePointerText.innerHTML = `<h4>${d.getHours()}:${String(d.getMinutes()).padStart(2, '0')}</h4>`;
    timePointerText.style.top = t;
    timePointer.style.top = `calc(${verticalHeaderOffset} + ${t})`;
}
// #endregion

// #region toggles
function toggleItemContainer(i) {
    let e = i.parentElement;
    e.ariaLabel = e.ariaLabel == "open" ? "closed" : "open";
}

function toggleTaskSelection() {
    let r = document.querySelector("#create");
    r.ariaLabel = r.ariaLabel == 'open' ? 'closed' : 'open';
}

function toggleTaskCreation(s) {
    if (s === false) {
        creationContainer.ariaLabel = "closed";
        return;
    }
    creationContainer.ariaLabel = 'open';

    creationContainer.style.top = `calc(${document.querySelector("#main #header-container").clientHeight}px + var(--task-height) * ${fetchCurrentTime()})`;
    creationContainer.style.left = `calc(5ch + calc(var(--task-width) * ${fetchDayIndex()}))`;

    creationContainer.scrollIntoView({
        'behavior':'auto',
        'block':'center',
        'inline':'center'
    });

    creationContainer.dataset.species = s;
    document.querySelector("#create").ariaLabel = 'closed';
}

function toggleColourPicker() {
    let d = document.querySelector("#task-creation #colour-picker");
    d.ariaLabel = d.ariaLabel == "open" ? "closed" : "open";
}

function toggleEndContainer(b) {
    document.querySelector("#main #task-creation #end-container").ariaLabel = b ? 'open' : 'closed';
}
// #endregion

// #region account related
document.querySelector("#sidebar #user-data > div h4:first-of-type").innerHTML = username;
// #endregion

// #region task creation
// #region colour
function initializeColours() {
    let d = document.querySelector("#colour-picker > div");
    d.innerHTML = "";
    colour_themes.forEach((c, index) => {
        d.innerHTML += `<hr onclick="selectColourTheme(${index})" style="background:${c}">`;
    })
}

initializeColours();

function selectColourTheme(i) {
    creationContainer.dataset.colour_theme = i;
    creationContainer.style.background = colour_themes[i];

    toggleColourPicker();
}
// #endregion

function verifyValidity() {
    let t = document.querySelector("#task-creation #title").value;
    let b = document.querySelector("#task-creation #create");
    b.ariaLabel = t ? 'enabled' : 'disabled';
}

function cleanTimeInputAll() {
    let l = document.querySelectorAll('#task-creation #time input[type="number"]');
    cleanTimeInput(l[0]);
    cleanTimeInput(l[1], true);
    cleanTimeInput(l[2]);
    cleanTimeInput(l[3], true);
    // making a loop checking odd/even would only make the code harder to understand
    // keep it stupid simple
}

function addTask(e) {
    if (e.ariaLabel == "disabled") {
        return;
    }

    toggleTaskCreation(false);
    cleanTimeInputAll(); // in case changed through inspector
    // /<r_species>/<r_time_species>/<repeating_day>/<title>/<description>/<colour>/<start>/<end>
    let r_species = document.querySelector("#task-creation #type-selection > div > label > input").checked ? "Task" : "Event";

    let repeating_day = 0;
    document.querySelectorAll("#task-creation #repeat > div input").forEach((e, index) => {
        repeating_day += (e.checked ? 1 : 0) * (Math.pow(2, (8 - index))); // invert to become big endian
    })
    let r_occurance_species = creationContainer.dataset.species;

    let title = document.querySelector("#task-creation #title").value;
    let description = document.querySelector("#task-creation #description").value;
    description = description ? description : ' ';
    let colour = creationContainer.dataset.colour_theme;
    // let start = document.querySelector("#task-creation")
    var temp = document.querySelectorAll(`#task-creation #time #start input[type="number"]`);
    var start = (temp[0].value * 3600) + (temp[1].value * 60);
    var temp = document.querySelectorAll(`#task-creation #time #end input[type="number"]`);
    var end = (temp[0].value * 3600) + (temp[1].value * 60);

    let all_day_checked = document.querySelector("#time #all-day input").checked;

    if (r_occurance_species != "Repeating") {
        if (!all_day_checked) {
            // add date epochs to start and end
            let date_epoch = document.querySelector("#task-creation #time #start input[type='date']").valueAsNumber / 1000;
            start += date_epoch ? date_epoch : (getEpochDate(true) * 86400);
            date_epoch = document.querySelector("#task-creation #time #end input[type='date']").valueAsNumber / 1000;
            end += date_epoch ? date_epoch : (getEpochDate(true) * 86400);
        } else {
            // only take the epoch dates
            let date_epoch = document.querySelector("#task-creation #time #start input[type='date']").valueAsNumber / 1000;
            start = date_epoch ? date_epoch : (getEpochDate(false) * 86400);
            date_epoch = document.querySelector("#task-creation #time #end input[type='date']").valueAsNumber / 1000;
            end = date_epoch ? date_epoch : (getEpochDate(false) * 86400);
        }
        // convoluted but thats for a future refactor to worry about 😊
    }

    let r_time_species = all_day_checked ? (start == end ? 'AllDay' : 'DayRange') : (start == end ? 'Start' : 'Range');

    if (r_occurance_species == "Repeating") {
        if (r_time_species == 'DayRange') {
            r_time_species = 'AllDay';
        }
    }

    [end, start] = end > start ? [end, start] : [start, end];
    // swap if the task ends before it starts
    let params = `${r_species}/${r_time_species}/${start}/${end}/${r_occurance_species}/${repeating_day}/${encodeURIComponent(title)}/${encodeURIComponent(description)}/${colour}`;
    sendPostRequest(`${BACKEND_ADDRESS}/add_task/${params}`, login_info(), (r) => {
        parseResponse(r);
        fetchLibrary();
    })
}
// #endregion

var timerHandler = setInterval(() => {
    setTimePointer();
}, 250);