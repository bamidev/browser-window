var cmd_line = ""
var is_executing = false;
var working_dir = "C:\\"



function addCmdChar( char_code ) {
	let char = String.fromCharCode( char_code )
	cmd_line += char

	if ( !is_executing )
		print( char )
}

function executeCmdLine() {

	// If not executing something, use the command line to execute something
	if ( !is_executing ) {
		is_executing = true
		invoke_extern("exec", cmd_line)
	}
	// If already executing something, use the line as input
	else
		invoke_extern("input", cmd_line)

	// Empty the buffer
	cmd_line = ""
}

function initialize( wd ) {
    working_dir = wd;

    printPrefix()
}

function onExecutionEnded() {
	printPrefix()
	print( cmd_line )
	is_executing = false
}

function onOutputReceived( output ) {
	let span = document.createElement("span")
	span.innerText = output

	document.body.appendChild( span )
}

function onErrorOutputReceived( output ) {

	let span = document.createElement("span")
	span.setAttribute("class", "stderr")
	span.innerText = output

	document.body.appendChild( span )
}

function print( text ) {
	$('body').append( text )
}

function printPrefix() {
	print( working_dir + '$ ' )
}



window.onkeypress = e => {

	if ( e.charCode == 0x0D ) {
	    print("\r\n")
		executeCmdLine()
    }
	else
		addCmdChar( e.charCode )
}