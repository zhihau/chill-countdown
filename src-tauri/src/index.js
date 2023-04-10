const tauri = require('tauri/api/tauri')
const inputDate = document.getElementById('input-date')
const inputTime = document.getElementById('input-time')
const remainingTime = document.getElementById('remaining-time')
const submitButton = document.getElementById('submit-button')
const menuQuit = document.getElementById('menu-quit')

menuQuit.addEventListener('click', () => {
  tauri.quit()
})

submitButton.addEventListener('click', async () => {
  const date = inputDate.value
  const time = inputTime.value
  const endTimestamp = new Date(`${date}T${time}`).getTime() / 1000
  const remainingTimeString = await tauri.promisified({
    cmd: 'get_remaining_time',
    args: [endTimestamp],
  })
  remainingTime.innerText = remainingTimeString
})
