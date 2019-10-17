function goTop(minHeight){
    $("#scroll-btn").click(
        function(){$('html,body').animate({scrollTop:'0px'},'normal');
    })
    minHeight? minHeight = minHeight:minHeight = 600;
    $(window).scroll(function(){
        var s = $(window).scrollTop();
        if( s > minHeight){
            $("#scroll-btn").fadeIn(300);
        }else{
            $("#scroll-btn").fadeOut(300);
        };
    });
}
window.onload = goTop();

scrollBar();
window.onresize = () => scrollBar();
function scrollBar() {
  let pageHeight = document.body.scrollHeight || document.documentElement.scrollHeight;
  let windowHeight = document.documentElement.clientHeight || document.body.clientHeight;
  let scrollAvail = pageHeight - windowHeight;
  window.onscroll = function() {
    let scrollTop = document.documentElement.scrollTop || document.body.scrollTop;
    document.querySelector('.scrollBar').style.width = (scrollTop/scrollAvail) * 100 + '%';
  };
}