from datetime import datetime

html_start = """
	<!DOCTYPE html>
	<html>
	    <head>
	        <meta charset="utf-8" />
	        <title>Foosball Tournament LIS - Summer 2025 Edition</title>
	        <link rel="stylesheet" href="https://cdn.simplecss.org/simple.min.css">
	        <link rel="stylesheet" href="style.css">
	    </head>
	 

	    <body>
			<nav>
		    
		        <a href="index.html">General informations</a>
		        <a href="indiv_ranking.html">Individual ranking</a>
		        <a href="team_ranking.html">Team ranking</a>
		        <a href="data.html">Games</a>
		        <a href="stats.html">Statistics</a>
		    
			</nav>
"""
html_end = """
	    </body>
	</html>
"""


def html_table(col_name, data):
	names = "<tr>{}</tr>".format(" ".join([f"<th>{i}</th>" for i in col_name]))
	items = "\n".join(["<tr>{}</tr>".format(" ".join([f"<td>{i}</td>" for i in l])) for l in data])
	return f"<table>\n{names}\n{items}</table>"

def html_page(body):
	global html_start, html_end
	today = datetime.now()
	return f"{html_start}\n{body}<p>Last update: {today}</p>{html_end}"