import ranking.reader as reader
import html_writer as hw


def update_matches(data):
	items = [i.get_data() for i in data.matches]
	names = ["Team 1", "Team 2", "Points of team 1", "Points of team 2", "Date"]
	with open("data.html", "w") as f:
		f.write(hw.html_table(names, items))

def update_indiv_ranking(data):
	items = data.get_indiv_ranking()
	names = ["Player", "Elo", "Rank", "Games played", "Games won", "Games lost", "Winning ratio"]
	with open("indiv_ranking.html", "w") as f:
		f.write(hw.html_table(names, items))

data = reader.read()

update_matches(data)
update_indiv_ranking(data)
  