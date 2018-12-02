mergeInto(LibraryManager.library, {
  hide_loading: function(num) {
    var loading = document.getElementById('loading')
    loading.parentNode.removeChild(loading)
  },
  play_audio: function(num) {
    var audio = []
    audio[0] = document.getElementById('audio_1')
    audio[1] = document.getElementById('audio_2')
    audio[2] = document.getElementById('audio_3')
    audio[3] = document.getElementById('audio_4')
    for (var i = 0; i < audio.length; i++) {
      if (i === num) {
        audio[i].play()
      } else {
        audio[i].pause()
        audio[i].currentTime = 0
      }
    }
  },
})
