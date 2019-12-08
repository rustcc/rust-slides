var chartSection = document.getElementById("s-pop-chart");

Reveal.addEventListener( 'slidechanged', function( event ) {

    console.log(event.currentSlide);
    if (!event.currentSlide || event.currentSlide.id != "s-pop-chart") {
        return;
    }
    
    var chart = new CanvasJS.Chart("pop-chart", {
	    animationEnabled: true,
	    title: {
		    text: "Population - China v. Europe"
	    },
        theme: "dark1",
        backgroundColor: "#202024",
        animationEnabled: false,
	    data: [{
		    type: "pie",
		    startAngle: 60,
		    yValueFormatString: "##0.0\" billion\"",
		    indexLabel: "{label} {y}",
		    dataPoints: [
			    {y: 1.4, label: "China", color: "#DE2910"},
			    /*{y: 7.7, label: "World", color: "blue"},*/
			    {y: 0.7, label: "Europe", color: "#1029DE"}
		    ]
	    }]
    });
    chart.render();    

} );
