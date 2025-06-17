import ranking.reader as reader
from ranking.team_ranking import Instance
import html_writer as hw
import networkx as nx
import matplotlib.pyplot as plt
import numpy as np
import seaborn as sns


def update_matches(data):
	items = [i.get_data() for i in data.matches]
	names = ["Team 1", "Team 2", "Points of team 1", "Points of team 2", "Date"]
	with open("data.html", "w") as f:
		f.write(hw.html_page(hw.html_table(names, items)))

def update_indiv_ranking(data):
	items = data.get_indiv_ranking()
	names = ["Player", "Elo", "Rank", "Games played", "Games won", "Games lost", "Winning ratio"]
	with open("indiv_ranking.html", "w") as f:
		f.write(hw.html_page(hw.html_table(names, items)))

def update_team_ranking(data):
	names = ["Rank", "Team", "Games played", "Games won", "Games lost", "Winning ratio"]
	l = list(nx.strongly_connected_components(data.graph))
	rankings = []
	for i in range(len(l)):
		inst = Instance(data.graph, l[i])
		r = inst.solve()
		items = [[j+1] + data.get_team_stats(r[j]) for j in range(len(r))]
		if len(r) > 0:
			rankings.append(f"<h1>Group {i+1}</h1>")
			rankings.append(hw.html_table(names, items))

	with open("team_ranking.html", "w") as f:
		f.write(hw.html_page("\n".join(rankings)))

def get_team_names(team):
	s = f"{str(team.players[0])}\n{str(team.players[1])}"
	size = max(len(str(team.players[0])), len(str(team.players[1])))
	return s, size**2 * 60


data = reader.read()


edgelist = list(set([e if data.graph[e[0]][e[1]]["score"] > 0.5 else (e[1],e[0]) for e in data.graph.edges()]))
update_matches(data)
update_indiv_ranking(data)
update_team_ranking(data)

node_labels = {u:get_team_names(u) for u in data.graph.nodes()}
pos = nx.spring_layout(data.graph, k=500, iterations=1000, scale = 20)
nx.draw(data.graph, pos, node_color="gray", node_size=[node_labels[u][1] for u in data.graph.nodes()], edgelist=edgelist)
nx.draw_networkx_labels(data.graph, pos, {u:node_labels[u][0] for u in data.graph.nodes()})
nx.draw_networkx_edge_labels(data.graph, pos, {e:np.round(data.graph[e[0]][e[1]]["score"],2) for e in edgelist})
plt.savefig("graph.svg")




df = data.get_indiv_stats()
df["ratio"] = 100*df["won"]/df["games"]
for target in ["games", "ratio", "elo"]:
	plt.figure(figsize=(25,15))
	sns.lineplot(data=df, x="date", y=target, hue="player")
	plt.savefig(f"{target}.svg")
