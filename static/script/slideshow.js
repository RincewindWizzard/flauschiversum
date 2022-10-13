/*$(function() {
  var aspect_ratio = 0.75
  function resizeImages() {
    $('.slideshow a').width( $('.slideshow').width() + 1)
    $('.slideshow').each(function() {
      var width = $(this).width()
      $(this).height(width * aspect_ratio)
    })
  }
  resizeImages()
  $(window).resize(function() {
    resizeImages()
  });
})*/

$(function() {
  /*$('div.slideshow').each(function(i, elem) {
    $(elem).attr("id","slideshow_" + i);
    $(elem).children('nav').attr('id', 'slideshow_nav_' + i)
    $("#slideshow_" + i).responsiveSlides({
      auto: false,
      pager: true,
      nav: true,
      speed: 0,
      namespace: "slideshow",
      //navContainer: '#slideshow_nav_' + i
    });    
  })*/
      $('.rslides').responsiveSlides({
      auto: false,
      pager: true,
      nav: true,
      speed: 0,
      namespace: "slideshow",
      navContainer: ''
    });  
});
