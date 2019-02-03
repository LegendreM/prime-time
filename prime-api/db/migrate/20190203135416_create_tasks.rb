class CreateTasks < ActiveRecord::Migration[5.2]
  def change
    create_table :tasks do |t|
      t.bigint :number
      t.string :state, :default => "pending"
      t.string :result, :default => ""

      t.timestamps
    end
  end
end
