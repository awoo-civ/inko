# frozen_string_literal: true

module Inkoc
  module AST
    class Method
      include Inspect

      attr_reader :name, :arguments, :type_arguments, :returns, :throws,
                  :body, :location

      # name - The name of the method.
      # args - The arguments of the method.
      # returns - The return type of the method.
      # throws - The type being thrown by this method.
      # body - The body of the method.
      # loc - The SourceLocation of this method.
      def initialize(name, args, targs, returns, throws, body, loc)
        @name = name
        @arguments = args
        @type_arguments = targs
        @returns = returns
        @throws = throws
        @body = body
        @location = loc
      end

      def required?
        @body.nil?
      end

      def tir_process_node_method
        :on_method
      end
    end
  end
end
