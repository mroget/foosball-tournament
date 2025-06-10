import ranking.reader
import networkx as nx
import subprocess

class Instance:
	def __init__(self, graph, allowed_teams):
		self.graph = graph
		degree = dict(nx.degree(self.graph))
		self.teams = [u for u in degree if degree[u] >= 4 and u in allowed_teams]
		self.teams_id = {self.teams[i] : i for i in range(len(self.teams))}
		self.votes = []
		for t in self.teams:
			self.process_team(t)

	def process_team(self, team):
		ret = []
		for opp in self.graph[team]:
			if opp in self.teams:
				ret.append((self.teams_id[opp], self.graph[team][opp]["score"]))
			ret.append((self.teams_id[team], 0.5))
		ret.sort(key=lambda x:x[1])
		self.votes.append([i[0] for i in ret])

	def get_instance(self):
		candidates = len(self.teams)
		votes = "\n".join([" ".join(list(map(str,v))) for v in self.votes])
		return f"{candidates}\n{candidates}\n{votes}"

	def solve(self):
		path = "./ranking/target/release/ranking"
		output = subprocess.run([path, "-v 0", "-s 5"], input=self.get_instance(), capture_output=True, text=True).stdout
		ranking = list(map(int,output.split(":")[1].strip().split(" ")))
		ranking = [self.teams[i].id for i in ranking]
		return ranking