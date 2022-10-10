const { contextBridge, ipcRenderer } = require('electron')

contextBridge.exposeInMainWorld('electronAPI',{
    data: () => ipcRenderer.invoke('chart:last_ping'),
    delay: (ip) => ipcRenderer.invoke('awesome:delay', ip),

})