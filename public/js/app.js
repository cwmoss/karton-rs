console.log("mounted!")

/*
https://stackoverflow.com/questions/25308823/targeting-positionsticky-elements-that-are-currently-in-a-stuck-state
*/
if (typeof IntersectionObserver !== 'function') {
    // sorry, IE https://caniuse.com/#feat=intersectionobserver
    document.querySelector('header').classList.add('unsticky')

}else{
    const observer = new IntersectionObserver( 
        ([e]) => 
          e.target.toggleAttribute('stuck', e.intersectionRatio < 1),
          {threshold: [1]}
      );
      
    observer.observe(document.querySelector('header'));
}

const lightbox = GLightbox({ selector:'.ltbx' })

console.log('++ lb', lightbox)