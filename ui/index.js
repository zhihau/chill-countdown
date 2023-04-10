//const tauri = require('tauri/api/tauri')
const { invoke } = window.__TAURI__.tauri
const inputDate = document.getElementById('input-date')
const inputTime = document.getElementById('input-time')
const remainingTime = document.getElementById('remaining-time')
const submitButton = document.getElementById('submit-button')
const menuQuit = document.getElementById('menu-quit')

menuQuit.addEventListener('click', () => {
  window.__TAURI__.tauri.quit()
})

submitButton.addEventListener('click', async () => {
  const date = inputDate.value
  const time = inputTime.value
  const endTimestamp = new Date(`${date}T${time}`).getTime() / 1000
console.log(endTimestamp)
invoke('get_remaining_time', { endTime: endTimestamp })
        // `invoke` returns a Promise
        .then((response) => {
console.log(response);
remainingTime.innerText= response;
//document.getElementById('title').innerHTML = response;
          //window.header.innerHTML = response
        })
  
})
