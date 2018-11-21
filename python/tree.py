#!/usr/bin/python

import sys

# Only return this if we want the parent to delete us
DELETE_NODE = 1
NO_DELETE_NODE = 0

class TreeNode:
	def __init__(self, x):
		self.val = x
		self.left_node = None
		self.right_node = None

	def insert(self, x):
		if (self.val == x):
			return
		elif (x < self.val):
			if (not self.left_node):
				self.left_node = TreeNode(x)
			else:
				self.left_node.insert(x)
		elif (x > self.val):
			if (not self.right_node):
				self.right_node = TreeNode(x)
			else:
				self.right_node.insert(x)

	def find(self, x):
		if (self.val == x):
			return self.val
		elif (x < self.val and self.left_node):
			return self.left_node.find(x)
		elif (x > self.val and self.right_node):
			return self.right_node.find(x)
		else:
			return None

	def delete(self, x):
		# If we find the node, stop and do delete here
		if (self.val == x):
			# Clear value
			self.val = 0

			# Case where left and right nodes are null
			if (not self.left_node and not self.right_node):
				return DELETE_NODE
			
			# Case where we only have a left node
			if (self.left_node and not self.right_node):
				self.val = self.left_node.val
				self.left_node = None
				return NO_DELETE_NODE
			
			# Case where we only have a right node
			if (self.right_node and not self.left_node):
				self.val = self.right_node.val
				self.right_node = None
				return NO_DELETE_NODE
			
			# Case where we have two nodes
			if (self.right_node and self.left_node):
				# We want to use the value of the left_node since it will keep
				# keep the tree correct
				self.val = self.left_node.val
				self.left_node = None
				return NO_DELETE_NODE

		# If we did not find the value go down the tree and call delete recursively
		elif (x < self.val and self.left_node):
			if (self.left_node.delete(x) == DELETE_NODE):
				self.left_node = None
		elif (x > self.val and self.right_node):
			if (self.right_node.delete(x) == DELETE_NODE):
				self.right_node = None
		else:
			return None

	def print_val(self):
		sys.stdout.write(str(self.val) + " ")

	def print_tree(self):
		self.print_val()
		if (self.left_node):
			self.left_node.print_tree()
		if (self.right_node):
			self.right_node.print_tree()

# Create the tree root first
tree_root = TreeNode(10)
tree_root.insert(10)
tree_root.insert(8)
tree_root.insert(12)
tree_root.insert(9)
tree_root.insert(11)
tree_root.insert(7)

tree_root.print_tree()
print("")

# Test find
print("find for 9 is " + str(tree_root.find(9)))
print("find for 20 is " + str(tree_root.find(20)))

# Test delete edge node
tree_root.delete(12)
tree_root.print_tree()

# Test delete node with two edge nodes
print("")
tree_root.delete(8)
tree_root.print_tree()
print("")
